//! Validation utilities for the cutl CLI
//!
//! Validates URLs, short codes, and TTL formats.

use anyhow::{bail, Context};

/// Validates that a URL is well-formed and safe
///
/// # Rules
/// - Must start with `http://` or `https://`
/// - Cannot point to `localhost` or `127.0.0.1`
pub fn validate_url(url: &str) -> anyhow::Result<()> {
    // Check that URL starts with http:// or https://
    if !url.starts_with("http://") && !url.starts_with("https://") {
        bail!("URL must start with http:// or https://");
    }

    // Try to parse as URL to validate further
    let parsed = url::Url::parse(url).context("Invalid URL format")?;

    // Reject localhost and 127.0.0.1
    let host = parsed.host_str().unwrap_or("");
    if host == "localhost" || host.starts_with("127.0.0.1") {
        bail!("URL cannot point to localhost or 127.0.0.1");
    }

    Ok(())
}

/// Validates that a custom code matches the required pattern
///
/// # Rules
/// - Length: 1-32 characters
/// - Characters: only alphanumeric, hyphen, and underscore
pub fn validate_code(code: &str) -> anyhow::Result<()> {
    // Check length
    if code.is_empty() {
        bail!("Code cannot be empty");
    }
    if code.len() > 32 {
        bail!("Code cannot exceed 32 characters");
    }

    // Check characters: only alphanumeric, hyphen, and underscore
    if !code
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        bail!("Code can only contain letters, numbers, hyphens, and underscores");
    }

    Ok(())
}

/// Validates the format of a TTL string
///
/// # Supported formats
/// - `5s` - 5 seconds
/// - `5m` - 5 minutes
/// - `1h` - 1 hour
/// - `1d` - 1 day
pub fn validate_ttl_format(ttl: &str) -> anyhow::Result<()> {
    let ttl = ttl.trim().to_lowercase();

    if ttl.len() < 2 {
        bail!("Invalid TTL format. Use format like 5m, 1h, 3d");
    }

    let (num_str, unit) = ttl.split_at(ttl.len() - 1);

    // Check that the number part is valid
    num_str.parse::<u64>().context("Invalid TTL number")?;

    // Check that the unit is valid
    match unit {
        "s" | "m" | "h" | "d" => Ok(()),
        _ => bail!("Invalid TTL unit: {}. Use s, m, h, or d", unit),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_url_valid() {
        assert!(validate_url("https://example.com").is_ok());
        assert!(validate_url("http://example.com").is_ok());
    }

    #[test]
    fn test_validate_url_invalid() {
        assert!(validate_url("ftp://example.com").is_err());
        assert!(validate_url("localhost").is_err());
        assert!(validate_url("https://localhost").is_err());
        assert!(validate_url("https://127.0.0.1").is_err());
    }

    #[test]
    fn test_validate_code_valid() {
        assert!(validate_code("abc").is_ok());
        assert!(validate_code("ABC-123_test").is_ok());
        assert!(validate_code("a").is_ok());
        assert!(validate_code("a".repeat(32).as_str()).is_ok());
    }

    #[test]
    fn test_validate_code_invalid() {
        assert!(validate_code("").is_err());
        assert!(validate_code("a".repeat(33).as_str()).is_err());
        assert!(validate_code("abc@def").is_err());
        assert!(validate_code("abc def").is_err());
    }

    #[test]
    fn test_validate_ttl_format_valid() {
        assert!(validate_ttl_format("5s").is_ok());
        assert!(validate_ttl_format("5m").is_ok());
        assert!(validate_ttl_format("1h").is_ok());
        assert!(validate_ttl_format("3d").is_ok());
    }

    #[test]
    fn test_validate_ttl_format_invalid() {
        assert!(validate_ttl_format("5").is_err());
        assert!(validate_ttl_format("1w").is_err());
        assert!(validate_ttl_format("abc").is_err());
    }
}
