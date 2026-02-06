//! Configuration management for the cutl CLI
//!
//! Loads configuration from command-line arguments and environment variables.

use std::env;

/// CLI configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// The URL to shorten
    pub url: String,

    /// Optional custom short code
    pub code: Option<String>,

    /// Optional time-to-live
    pub ttl: Option<String>,

    /// Server API URL
    pub server_url: String,

    /// Optional auth token
    pub auth_token: Option<String>,
}

impl Config {
    /// Load configuration from arguments and environment variables
    ///
    /// # Arguments
    /// * `url` - The URL to shorten
    /// * `code` - Optional custom short code
    /// * `ttl` - Optional time-to-live
    /// * `server` - Optional server URL override
    pub fn new(url: String, code: Option<String>, ttl: Option<String>, server: Option<String>) -> Self {
        let server_url = server
            .or_else(|| env::var("CUTL_SERVER").ok())
            .unwrap_or_else(|| "https://cutl.my.id".to_string());

        let auth_token = env::var("CUTL_TOKEN").ok();

        Self {
            url,
            code,
            ttl,
            server_url,
            auth_token,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new_basic() {
        let config = Config::new(
            "https://example.com".to_string(),
            None,
            None,
            None,
        );
        assert_eq!(config.url, "https://example.com");
        assert!(config.code.is_none());
        assert!(config.ttl.is_none());
        assert_eq!(config.server_url, "https://cutl.my.id");
        assert!(config.auth_token.is_none());
    }

    #[test]
    fn test_config_new_with_code() {
        let config = Config::new(
            "https://example.com".to_string(),
            Some("mycode".to_string()),
            None,
            None,
        );
        assert_eq!(config.code, Some("mycode".to_string()));
    }

    #[test]
    fn test_config_new_with_ttl() {
        let config = Config::new(
            "https://example.com".to_string(),
            None,
            Some("1h".to_string()),
            None,
        );
        assert_eq!(config.ttl, Some("1h".to_string()));
    }

    #[test]
    fn test_config_new_with_server_override() {
        let config = Config::new(
            "https://example.com".to_string(),
            None,
            None,
            Some("http://custom.server:8080".to_string()),
        );
        assert_eq!(config.server_url, "http://custom.server:8080");
    }

    #[test]
    fn test_config_server_url_trailing_slash() {
        let config = Config::new(
            "https://example.com".to_string(),
            None,
            None,
            Some("http://localhost:3000/".to_string()),
        );
        assert_eq!(config.server_url, "http://localhost:3000/");
    }

    #[test]
    fn test_config_all_fields() {
        let config = Config::new(
            "https://example.com".to_string(),
            Some("test".to_string()),
            Some("7d".to_string()),
            Some("http://server:3000".to_string()),
        );
        assert_eq!(config.url, "https://example.com");
        assert_eq!(config.code, Some("test".to_string()));
        assert_eq!(config.ttl, Some("7d".to_string()));
        assert_eq!(config.server_url, "http://server:3000");
    }

    #[test]
    fn test_config_empty_code_becomes_none() {
        let config = Config::new(
            "https://example.com".to_string(),
            Some("".to_string()),
            None,
            None,
        );
        // Empty string is still Some(""), not None
        assert_eq!(config.code, Some("".to_string()));
    }
}
