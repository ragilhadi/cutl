//! Data models for the cutl server
//!
//! Defines request/response types and domain models.

use axum::{http::StatusCode, response::{IntoResponse, Json}};
use serde::{Deserialize, Serialize};

/// Application state shared across all request handlers
#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::Pool<sqlx::Sqlite>,
    pub base_url: String,
    pub auth_token: Option<String>,
}

/// Request body for creating a shortened URL
#[derive(Debug, Deserialize)]
pub struct ShortenRequest {
    /// Original URL to shorten
    pub url: String,

    /// Optional custom short code (1-32 chars, alphanumeric + - and _)
    pub code: Option<String>,

    /// Optional TTL (e.g., "5m", "1h", "3d", "30d")
    pub ttl: Option<String>,
}

/// Response after successfully creating a short link
#[derive(Debug, Serialize)]
pub struct ShortenResponse {
    /// The short code
    pub code: String,

    /// Full short URL
    pub short_url: String,

    /// Expiration timestamp (UNIX seconds)
    pub expires_at: i64,
}

/// Error response type
#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub message: String,
}

impl ApiError {
    /// Create a new API error
    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
        }
    }

    /// Bad request (400)
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, message)
    }

    /// Unauthorized (401)
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, message)
    }

    /// Not found (404)
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, message)
    }

    /// Conflict (409)
    pub fn conflict(message: impl Into<String>) -> Self {
        Self::new(StatusCode::CONFLICT, message)
    }

    /// Internal server error (500)
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, message)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (self.status, Json(serde_json::json!({"error": self.message}))).into_response()
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        Self::internal(format!("Database error: {}", err))
    }
}

/// Database record for a shortened link
#[derive(Debug)]
#[allow(dead_code)]
pub struct Link {
    pub code: String,
    pub original_url: String,
    pub expires_at: i64,
    pub created_at: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error_new() {
        let error = ApiError::new(StatusCode::BAD_REQUEST, "Test error");
        assert_eq!(error.status, StatusCode::BAD_REQUEST);
        assert_eq!(error.message, "Test error");
    }

    #[test]
    fn test_api_error_bad_request() {
        let error = ApiError::bad_request("Invalid input");
        assert_eq!(error.status, StatusCode::BAD_REQUEST);
        assert_eq!(error.message, "Invalid input");
    }

    #[test]
    fn test_api_error_unauthorized() {
        let error = ApiError::unauthorized("Missing token");
        assert_eq!(error.status, StatusCode::UNAUTHORIZED);
        assert_eq!(error.message, "Missing token");
    }

    #[test]
    fn test_api_error_not_found() {
        let error = ApiError::not_found("Resource not found");
        assert_eq!(error.status, StatusCode::NOT_FOUND);
        assert_eq!(error.message, "Resource not found");
    }

    #[test]
    fn test_api_error_conflict() {
        let error = ApiError::conflict("Duplicate entry");
        assert_eq!(error.status, StatusCode::CONFLICT);
        assert_eq!(error.message, "Duplicate entry");
    }

    #[test]
    fn test_api_error_internal() {
        let error = ApiError::internal("Database failure");
        assert_eq!(error.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(error.message, "Database failure");
    }

    #[test]
    fn test_api_error_into_response() {
        let error = ApiError::bad_request("Test error");
        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_api_error_message_types() {
        let error1 = ApiError::bad_request(String::from("String message"));
        let error2 = ApiError::bad_request("&str message");

        assert_eq!(error1.message, "String message");
        assert_eq!(error2.message, "&str message");
    }

    #[test]
    fn test_shorten_request_deserialize() {
        let json = r#"{"url":"https://example.com","code":"test","ttl":"1h"}"#;
        let request: ShortenRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.url, "https://example.com");
        assert_eq!(request.code, Some("test".to_string()));
        assert_eq!(request.ttl, Some("1h".to_string()));
    }

    #[test]
    fn test_shorten_request_minimal() {
        let json = r#"{"url":"https://example.com"}"#;
        let request: ShortenRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.url, "https://example.com");
        assert!(request.code.is_none());
        assert!(request.ttl.is_none());
    }

    #[test]
    fn test_shorten_response_serialize() {
        let response = ShortenResponse {
            code: "abc123".to_string(),
            short_url: "http://localhost:3000/abc123".to_string(),
            expires_at: 1234567890,
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"code\":\"abc123\""));
        assert!(json.contains("\"short_url\":\"http://localhost:3000/abc123\""));
        assert!(json.contains("\"expires_at\":1234567890"));
    }

    #[test]
    fn test_link_debug() {
        let link = Link {
            code: "test".to_string(),
            original_url: "https://example.com".to_string(),
            expires_at: 1234567890,
            created_at: 1234567800,
        };

        let debug_str = format!("{:?}", link);
        assert!(debug_str.contains("test"));
        assert!(debug_str.contains("https://example.com"));
    }
}
