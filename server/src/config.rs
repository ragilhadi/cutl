//! Configuration management for the cutl server
//!
//! Loads configuration from environment variables with sensible defaults.

use anyhow::Result;
use std::env;

/// Server configuration loaded from environment variables
#[derive(Debug, Clone)]
pub struct Config {
    /// SQLite database file path (e.g., "sqlite:cutl.db")
    pub database_url: String,

    /// Base URL for generating short links (e.g., "http://localhost:3000")
    pub base_url: String,

    /// Address to bind the server to (e.g., "0.0.0.0:3000")
    pub bind_address: String,

    /// Optional bearer token for API authentication
    pub auth_token: Option<String>,
}

impl Config {
    /// Load configuration from environment variables
    ///
    /// Environment variables:
    /// - `DATABASE_URL`: SQLite database path (default: "sqlite:cutl.db")
    /// - `BASE_URL`: Base URL for short links (default: "http://localhost:3000")
    /// - `BIND_ADDRESS`: Server bind address (default: "0.0.0.0:3000")
    /// - `AUTH_TOKEN`: Optional bearer token for API auth
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:cutl.db".to_string()),
            base_url: env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string()),
            bind_address: env::var("BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0:3000".to_string()),
            auth_token: env::var("AUTH_TOKEN").ok(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_env_defaults() {
        // Clear all environment variables to ensure clean state
        std::env::remove_var("DATABASE_URL");
        std::env::remove_var("BASE_URL");
        std::env::remove_var("BIND_ADDRESS");
        std::env::remove_var("AUTH_TOKEN");

        let config = Config::from_env().unwrap();
        assert_eq!(config.database_url, "sqlite:cutl.db");
        assert_eq!(config.base_url, "http://localhost:3000");
        assert_eq!(config.bind_address, "0.0.0.0:3000");
        assert!(config.auth_token.is_none());
    }

    #[test]
    fn test_config_from_env_custom_database() {
        // Clear all env vars first
        std::env::remove_var("DATABASE_URL");
        std::env::remove_var("BASE_URL");
        std::env::remove_var("BIND_ADDRESS");
        std::env::remove_var("AUTH_TOKEN");

        std::env::set_var("DATABASE_URL", "sqlite:test.db");
        let config = Config::from_env().unwrap();
        assert_eq!(config.database_url, "sqlite:test.db");
        std::env::remove_var("DATABASE_URL");
    }

    #[test]
    fn test_config_from_env_custom_base_url() {
        // Clear all env vars first
        std::env::remove_var("DATABASE_URL");
        std::env::remove_var("BASE_URL");
        std::env::remove_var("BIND_ADDRESS");
        std::env::remove_var("AUTH_TOKEN");

        std::env::set_var("BASE_URL", "https://cutl.example.com");
        let config = Config::from_env().unwrap();
        assert_eq!(config.base_url, "https://cutl.example.com");
        std::env::remove_var("BASE_URL");
    }

    #[test]
    fn test_config_from_env_custom_bind_address() {
        // Clear all env vars first
        std::env::remove_var("DATABASE_URL");
        std::env::remove_var("BASE_URL");
        std::env::remove_var("BIND_ADDRESS");
        std::env::remove_var("AUTH_TOKEN");

        std::env::set_var("BIND_ADDRESS", "127.0.0.1:8080");
        let config = Config::from_env().unwrap();
        assert_eq!(config.bind_address, "127.0.0.1:8080");
        std::env::remove_var("BIND_ADDRESS");
    }

    #[test]
    fn test_config_from_env_with_auth_token() {
        // Clear all env vars first
        std::env::remove_var("DATABASE_URL");
        std::env::remove_var("BASE_URL");
        std::env::remove_var("BIND_ADDRESS");
        std::env::remove_var("AUTH_TOKEN");

        std::env::set_var("AUTH_TOKEN", "secret-token-123");
        let config = Config::from_env().unwrap();
        assert_eq!(config.auth_token, Some("secret-token-123".to_string()));
        std::env::remove_var("AUTH_TOKEN");
    }

    #[test]
    fn test_config_from_env_all_custom() {
        // Clear all env vars first
        std::env::remove_var("DATABASE_URL");
        std::env::remove_var("BASE_URL");
        std::env::remove_var("BIND_ADDRESS");
        std::env::remove_var("AUTH_TOKEN");

        std::env::set_var("DATABASE_URL", "sqlite:production.db");
        std::env::set_var("BASE_URL", "https://cutl.my.id");
        std::env::set_var("BIND_ADDRESS", "0.0.0.0:9000");
        std::env::set_var("AUTH_TOKEN", "prod-token");

        let config = Config::from_env().unwrap();
        assert_eq!(config.database_url, "sqlite:production.db");
        assert_eq!(config.base_url, "https://cutl.my.id");
        assert_eq!(config.bind_address, "0.0.0.0:9000");
        assert_eq!(config.auth_token, Some("prod-token".to_string()));

        // Cleanup
        std::env::remove_var("DATABASE_URL");
        std::env::remove_var("BASE_URL");
        std::env::remove_var("BIND_ADDRESS");
        std::env::remove_var("AUTH_TOKEN");
    }

    #[test]
    fn test_config_debug_clone() {
        let config = Config {
            database_url: "sqlite:test.db".to_string(),
            base_url: "http://localhost:3000".to_string(),
            bind_address: "0.0.0.0:3000".to_string(),
            auth_token: Some("token".to_string()),
        };

        // Test Clone trait
        let config2 = config.clone();
        assert_eq!(config.database_url, config2.database_url);

        // Test Debug trait
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("test.db"));
    }
}
