//! Utility functions for the cutl server
//!
//! Includes code generation, validation, and TTL parsing.

use rand::RngExt;
use regex::Regex;
use std::time::{SystemTime, UNIX_EPOCH};

/// Minimum TTL in seconds (5 minutes)
pub const MIN_TTL_SECONDS: i64 = 300;

/// Maximum TTL in seconds (30 days)
pub const MAX_TTL_SECONDS: i64 = 30 * 24 * 60 * 60;

/// Characters used for auto-generated short codes (base62)
const BASE62_CHARS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

lazy_static::lazy_static! {
    /// Regex for validating short codes
    static ref CODE_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_-]{1,32}$").unwrap();
}

/// Gets the current UNIX timestamp in seconds
pub fn now_unix() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

/// Generates a random base62 short code
///
/// Length is randomly chosen between 6-8 characters
pub fn generate_code() -> String {
    let mut rng = rand::rng();
    let length = rng.random_range(6..=8);

    (0..length)
        .map(|_| {
            let idx = rng.random_range(0..BASE62_CHARS.len());
            BASE62_CHARS[idx] as char
        })
        .collect()
}

/// Validates that a URL string is well-formed and safe
///
/// # Rules
/// - Must start with `http://` or `https://`
/// - Cannot point to `localhost` or `127.0.0.1`
pub fn validate_url(url: &str) -> anyhow::Result<()> {
    // Check that URL starts with http:// or https://
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(anyhow::anyhow!("URL must start with http:// or https://"));
    }

    // Reject localhost and 127.0.0.1
    let url_lower = url.to_lowercase();
    if url_lower.contains("localhost") || url_lower.contains("127.0.0.1") {
        return Err(anyhow::anyhow!(
            "URL cannot point to localhost or 127.0.0.1"
        ));
    }

    Ok(())
}

/// Validates a short code against the allowed pattern
///
/// # Rules
/// - Length: 1-32 characters
/// - Characters: alphanumeric, hyphen, underscore
/// - Pattern: `^[a-zA-Z0-9_-]{1,32}$`
pub fn validate_code(code: &str) -> anyhow::Result<()> {
    // Check length constraints
    if code.is_empty() {
        return Err(anyhow::anyhow!("Code cannot be empty"));
    }
    if code.len() > 32 {
        return Err(anyhow::anyhow!("Code cannot exceed 32 characters"));
    }

    // Check that code matches pattern: alphanumeric + - and _
    if !CODE_REGEX.is_match(code) {
        return Err(anyhow::anyhow!(
            "Code can only contain letters, numbers, hyphens, and underscores"
        ));
    }

    Ok(())
}

/// Parses a TTL string into seconds
///
/// # Supported formats
/// - `5s` - 5 seconds
/// - `5m` - 5 minutes
/// - `1h` - 1 hour
/// - `1d` - 1 day
/// - `30d` - 30 days
///
/// # Limits
/// - Minimum: 5 minutes (300 seconds)
/// - Maximum: 30 days (2,592,000 seconds)
pub fn parse_ttl(ttl: &str) -> anyhow::Result<i64> {
    let ttl = ttl.trim().to_lowercase();

    if ttl.len() < 2 {
        return Err(anyhow::anyhow!("Invalid TTL format"));
    }

    let (num_str, unit) = ttl.split_at(ttl.len() - 1);
    let num: i64 = num_str
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid TTL number: {}", num_str))?;

    let seconds = match unit {
        "s" => num,
        "m" => num * 60,
        "h" => num * 60 * 60,
        "d" => num * 24 * 60 * 60,
        _ => {
            return Err(anyhow::anyhow!(
                "Invalid TTL unit: {}. Use s, m, h, or d",
                unit
            ))
        }
    };

    // Validate range
    if seconds < MIN_TTL_SECONDS {
        return Err(anyhow::anyhow!(
            "TTL must be at least {} seconds (5 minutes)",
            MIN_TTL_SECONDS
        ));
    }
    if seconds > MAX_TTL_SECONDS {
        return Err(anyhow::anyhow!(
            "TTL cannot exceed {} seconds (30 days)",
            MAX_TTL_SECONDS
        ));
    }

    Ok(seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_code_length() {
        let code = generate_code();
        assert!(code.len() >= 6 && code.len() <= 8);
    }

    #[test]
    fn test_generate_code_unique() {
        let code1 = generate_code();
        let code2 = generate_code();
        assert_ne!(code1, code2);
    }

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
    fn test_parse_ttl_valid() {
        // Note: Minimum TTL is 5 minutes (300 seconds)
        assert!(parse_ttl("5s").is_err()); // Below minimum
        assert_eq!(parse_ttl("5m").unwrap(), 300); // At minimum
        assert_eq!(parse_ttl("1h").unwrap(), 3600);
        assert_eq!(parse_ttl("1d").unwrap(), 86400);
        assert_eq!(parse_ttl("30d").unwrap(), 30 * 24 * 60 * 60); // At maximum
    }

    #[test]
    fn test_parse_ttl_invalid() {
        assert!(parse_ttl("5").is_err());
        assert!(parse_ttl("1w").is_err()); // Invalid unit
        assert!(parse_ttl("abc").is_err());
        assert!(parse_ttl("1s").is_err()); // Below minimum
        assert!(parse_ttl("31d").is_err()); // Above maximum
        assert!(parse_ttl("4m").is_err()); // Below minimum (5 minutes)
    }

    #[test]
    fn test_parse_ttl_limits() {
        // Below minimum (5 minutes)
        assert!(parse_ttl("4m").is_err());

        // At minimum
        assert!(parse_ttl("5m").is_ok());

        // At maximum (30 days)
        assert!(parse_ttl("30d").is_ok());

        // Above maximum
        assert!(parse_ttl("31d").is_err());
    }

    #[test]
    fn test_now_unix() {
        let timestamp = now_unix();
        // Should be a reasonable timestamp (after 2020 and before 2100)
        assert!(timestamp > 1577836800); // Jan 1, 2020
        assert!(timestamp < 4102444800); // Jan 1, 2100
    }

    #[test]
    fn test_now_unix_increasing() {
        let ts1 = now_unix();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let ts2 = now_unix();
        assert!(ts2 >= ts1);
    }

    #[test]
    fn test_parse_ttl_case_insensitive() {
        // Note: Minimum TTL is 5 minutes (300 seconds)
        assert!(parse_ttl("5S").is_err()); // Below minimum
        assert_eq!(parse_ttl("5M").unwrap(), 300); // At minimum
        assert_eq!(parse_ttl("1H").unwrap(), 3600);
        assert_eq!(parse_ttl("1D").unwrap(), 86400);
    }

    #[test]
    fn test_parse_ttl_whitespace() {
        assert!(parse_ttl(" 5s ").is_err()); // Below minimum
        assert_eq!(parse_ttl(" 5m ").unwrap(), 300); // At minimum
        assert_eq!(parse_ttl("\t1h\t").unwrap(), 3600);
    }

    #[test]
    fn test_generate_code_only_base62() {
        for _ in 0..100 {
            let code = generate_code();
            assert!(code.chars().all(|c| c.is_alphanumeric()));
        }
    }

    #[test]
    fn test_validate_code_edge_cases() {
        // Single character codes
        assert!(validate_code("a").is_ok());
        assert!(validate_code("Z").is_ok());
        assert!(validate_code("0").is_ok());
        assert!(validate_code("-").is_ok());
        assert!(validate_code("_").is_ok());

        // Exactly 32 characters
        assert!(validate_code("a".repeat(32).as_str()).is_ok());

        // Special characters at edges
        assert!(validate_code("-abc").is_ok());
        assert!(validate_code("_abc").is_ok());
        assert!(validate_code("abc-").is_ok());
        assert!(validate_code("abc_").is_ok());
    }

    #[test]
    fn test_constants() {
        assert_eq!(MIN_TTL_SECONDS, 300); // 5 minutes
        assert_eq!(MAX_TTL_SECONDS, 30 * 24 * 60 * 60); // 30 days
    }
}
