//! cutl CLI - URL Shortener Client
//!
//! Command-line tool for creating shortened URLs via the cutl API.
//!
//! # Usage
//! ```bash
//! cutl <URL> [--ttl TTL] [--code CODE]
//! ```
//!
//! # Examples
//! ```bash
//! cutl https://example.com
//! cutl https://example.com --ttl 3d
//! cutl https://example.com --code docs --ttl 7d
//! ```

mod client;
mod config;
mod output;
mod validation;

use anyhow::Result;
use clap::Parser;

/// cutl - CLI URL Shortener
#[derive(Parser, Debug)]
#[command(name = "cutl")]
#[command(author = "cutl")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Shorten URLs using the cutl API", long_about = None)]
struct Args {
    /// The URL to shorten
    #[arg(value_name = "URL")]
    url: String,

    /// Optional: Custom short code (1-32 chars, alphanumeric + - and _)
    #[arg(short, long)]
    code: Option<String>,

    /// Optional: Time-to-live (e.g., 5m, 1h, 3d, 30d)
    #[arg(short, long)]
    ttl: Option<String>,

    /// Override the default server URL
    #[arg(short, long, env = "CUTL_SERVER")]
    server: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command-line arguments
    let args = Args::parse();

    // Validate the input URL
    validation::validate_url(&args.url)?;

    // Get server URL from args or environment variable
    let config = config::Config::new(args.url, args.code, args.ttl, args.server);

    // Validate custom code format if provided
    if let Some(ref code) = config.code {
        validation::validate_code(code)?;
    }

    // Validate TTL format if provided
    if let Some(ref ttl) = config.ttl {
        validation::validate_ttl_format(ttl)?;
    }

    // Create API client
    let client = client::ApiClient::new(config.server_url, config.auth_token)?;

    // Create a spinner for the request
    let spinner = output::create_spinner("Shortening URL...");

    // Send the request
    let result = match client.shorten(client::ShortenRequest {
        url: config.url,
        code: config.code,
        ttl: config.ttl,
    }).await {
        Ok(response) => response,
        Err(e) => {
            spinner.finish_and_clear();
            // Try to extract HTTP status from error message
            let status_code = extract_status_code(&e.to_string());
            output::print_error(&e.to_string(), status_code);
            return Err(e);
        }
    };

    spinner.finish_and_clear();

    // Format and display the result
    output::print_success(&result);

    Ok(())
}

/// Extract HTTP status code from error message if available
fn extract_status_code(error_msg: &str) -> u16 {
    // Look for common status code patterns in error messages
    if error_msg.contains("400") || error_msg.contains("Invalid") {
        400
    } else if error_msg.contains("401") || error_msg.contains("Unauthorized") {
        401
    } else if error_msg.contains("409") || error_msg.contains("exists") {
        409
    } else if error_msg.contains("404") {
        404
    } else if error_msg.contains("500") || error_msg.contains("Server error") {
        500
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_status_code_400() {
        assert_eq!(extract_status_code("Invalid request"), 400);
        assert_eq!(extract_status_code("400 Bad Request"), 400);
        assert_eq!(extract_status_code("Error 400"), 400);
    }

    #[test]
    fn test_extract_status_code_401() {
        assert_eq!(extract_status_code("Unauthorized"), 401);
        assert_eq!(extract_status_code("401 Unauthorized"), 401);
        assert_eq!(extract_status_code("401"), 401);
        assert_eq!(extract_status_code("Authentication failed"), 0); // No pattern match
    }

    #[test]
    fn test_extract_status_code_409() {
        assert_eq!(extract_status_code("Code already exists"), 409);
        assert_eq!(extract_status_code("409 Conflict"), 409);
        assert_eq!(extract_status_code("Resource exists"), 409);
    }

    #[test]
    fn test_extract_status_code_404() {
        assert_eq!(extract_status_code("404 Not Found"), 404);
        assert_eq!(extract_status_code("Resource not found"), 0); // No 404 in message
    }

    #[test]
    fn test_extract_status_code_500() {
        assert_eq!(extract_status_code("Server error"), 500);
        assert_eq!(extract_status_code("500 Internal Server Error"), 500);
        assert_eq!(extract_status_code("500"), 500);
        assert_eq!(extract_status_code("Internal error"), 0); // No pattern match
    }

    #[test]
    fn test_extract_status_code_unknown() {
        assert_eq!(extract_status_code("Some random error"), 0);
        assert_eq!(extract_status_code("Unknown failure"), 0);
        assert_eq!(extract_status_code("Network timeout"), 0);
    }

    #[test]
    fn test_extract_status_code_empty() {
        assert_eq!(extract_status_code(""), 0);
    }

    #[test]
    fn test_extract_status_code_case_insensitive() {
        // The function is case-sensitive, so uppercase won't match
        assert_eq!(extract_status_code("INVALID REQUEST"), 0);
        assert_eq!(extract_status_code("UNAUTHORIZED"), 0);
        assert_eq!(extract_status_code("SERVER ERROR"), 0); // Case-sensitive, "Server" != "SERVER"

        // Correct case works
        assert_eq!(extract_status_code("Invalid request"), 400);
        assert_eq!(extract_status_code("Unauthorized"), 401);
        assert_eq!(extract_status_code("Server error"), 500);
    }
}
