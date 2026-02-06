//! API client for the cutl CLI
//!
//! Handles communication with the cutl server API.

use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// API request to shorten a URL
#[derive(Serialize)]
pub struct ShortenRequest {
    pub url: String,
    pub code: Option<String>,
    pub ttl: Option<String>,
}

/// API response from the server
#[derive(Deserialize)]
pub struct ShortenResponse {
    pub code: String,
    pub short_url: String,
    pub expires_at: i64,
}

/// API error response
#[derive(Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// HTTP client for the cutl API
pub struct ApiClient {
    client: Client,
    server_url: String,
    auth_token: Option<String>,
}

impl ApiClient {
    /// Creates a new API client
    ///
    /// # Arguments
    /// * `server_url` - Base URL of the cutl server
    /// * `auth_token` - Optional bearer token for authentication
    pub fn new(server_url: String, auth_token: Option<String>) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;

        Ok(Self {
            client,
            server_url,
            auth_token,
        })
    }

    /// Sends a request to shorten a URL
    ///
    /// # Arguments
    /// * `request` - The shorten request containing URL, optional code, and TTL
    ///
    /// # Returns
    /// The server's response with the short URL details
    pub async fn shorten(&self, request: ShortenRequest) -> Result<ShortenResponse> {
        let api_url = format!("{}/shorten", self.server_url.trim_end_matches('/'));

        let mut req_builder = self.client.post(&api_url).json(&request);

        // Add auth token if available
        if let Some(ref token) = self.auth_token {
            req_builder = req_builder.bearer_auth(token);
        }

        let response = req_builder
            .send()
            .await
            .context("Failed to connect to server")?;

        let status = response.status();
        let response_text = response.text().await?;

        if status.is_success() {
            serde_json::from_str(&response_text)
                .context("Failed to parse server response")
        } else {
            let error_msg = if let Ok(err) = serde_json::from_str::<ErrorResponse>(&response_text) {
                err.error
            } else {
                format!("Server returned HTTP {}", status.as_u16())
            };

            anyhow::bail!("{}", error_msg);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shorten_request_serialization() {
        let request = ShortenRequest {
            url: "https://example.com".to_string(),
            code: Some("test".to_string()),
            ttl: Some("1h".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"url\":\"https://example.com\""));
        assert!(json.contains("\"code\":\"test\""));
        assert!(json.contains("\"ttl\":\"1h\""));
    }

    #[test]
    fn test_shorten_request_minimal() {
        let request = ShortenRequest {
            url: "https://example.com".to_string(),
            code: None,
            ttl: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"url\":\"https://example.com\""));
        assert!(json.contains("\"code\":null"));
        assert!(json.contains("\"ttl\":null"));
    }

    #[test]
    fn test_api_client_new() {
        let client = ApiClient::new("http://localhost:3000".to_string(), None);
        assert!(client.is_ok());
        let client = client.unwrap();
        assert_eq!(client.server_url, "http://localhost:3000");
        assert!(client.auth_token.is_none());
    }

    #[test]
    fn test_api_client_new_with_auth() {
        let client = ApiClient::new(
            "http://localhost:3000".to_string(),
            Some("secret-token".to_string()),
        );
        assert!(client.is_ok());
        let client = client.unwrap();
        assert_eq!(client.auth_token, Some("secret-token".to_string()));
    }

    #[test]
    fn test_api_client_trims_trailing_slash() {
        let client = ApiClient::new("http://localhost:3000/".to_string(), None).unwrap();
        assert_eq!(client.server_url, "http://localhost:3000/");
    }

    #[test]
    fn test_error_response_deserialization() {
        let json = r#"{"error":"Invalid URL"}"#;
        let response: ErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.error, "Invalid URL");
    }

    #[test]
    fn test_shorten_response_deserialization() {
        let json = r#"{"code":"abc123","short_url":"http://localhost:3000/abc123","expires_at":1234567890}"#;
        let response: ShortenResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, "abc123");
        assert_eq!(response.short_url, "http://localhost:3000/abc123");
        assert_eq!(response.expires_at, 1234567890);
    }
}
