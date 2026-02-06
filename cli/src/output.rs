//! Output formatting for the cutl CLI
//!
//! Handles styled terminal output for success and error messages.

use chrono::{DateTime, Local, SecondsFormat};
use console::Style;

/// Creates a styled progress spinner
pub fn create_spinner(message: &str) -> indicatif::ProgressBar {
    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));
    spinner.set_style(
        indicatif::ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner} {msg}")
            .unwrap(),
    );
    spinner.set_message(message.to_string());
    spinner
}

/// Prints a successful response with nice formatting
pub fn print_success(result: &crate::client::ShortenResponse) {
    let bold = Style::new().bold();
    let dim = Style::new().dim();
    let green = Style::new().green();

    println!();
    println!("{} {}", green.apply_to("✓"), bold.apply_to("Short URL created"));
    println!();
    println!("  {} {}", dim.apply_to("Short URL:"), bold.apply_to(&result.short_url));
    println!(
        "  {} {}",
        dim.apply_to("Code:"),
        bold.apply_to(&result.code)
    );

    // Format expiration timestamp
    let expires_dt: DateTime<Local> = DateTime::from_timestamp(result.expires_at, 0)
        .unwrap()
        .into();
    println!(
        "  {} {}",
        dim.apply_to("Expires:"),
        bold.apply_to(expires_dt.to_rfc3339_opts(SecondsFormat::Secs, false))
    );
    println!();
}

/// Prints an error message with appropriate styling
pub fn print_error(message: &str, status_code: u16) {
    let red = Style::new().red();
    let bold = Style::new().bold();

    eprintln!();
    eprintln!("{} {}", red.apply_to("✗"), bold.apply_to("Error"));

    // Add context based on status code
    let context = match status_code {
        400 => "Invalid request",
        401 => "Unauthorized - check your CUTL_TOKEN",
        409 => "Code already exists",
        500 => "Server error - try again later",
        _ => "Request failed",
    };

    eprintln!("  {} {}", Style::new().dim().apply_to(context), message);
    eprintln!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_spinner() {
        let spinner = create_spinner("Test message");
        // Just check that it doesn't panic - we can't easily inspect the spinner
        // The spinner is created with a message and should be valid
        drop(spinner); // Explicitly drop to avoid warnings
    }

    #[test]
    fn test_spinner_with_empty_message() {
        let spinner = create_spinner("");
        drop(spinner);
    }

    #[test]
    fn test_print_success_formatting() {
        let response = crate::client::ShortenResponse {
            code: "abc123".to_string(),
            short_url: "http://localhost:3000/abc123".to_string(),
            expires_at: 1735689600, // 2025-01-01 00:00:00 UTC
        };
        // Just check it doesn't panic - actual output testing would require capturing stdout
        print_success(&response);
    }

    #[test]
    fn test_print_error_various_codes() {
        print_error("Test error message", 400);
        print_error("Unauthorized", 401);
        print_error("Conflict", 409);
        print_error("Not found", 404);
        print_error("Server error", 500);
        print_error("Unknown error", 0);
        print_error("No code provided", 999);
    }
}
