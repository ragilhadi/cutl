//! HTTP request handlers for the cutl server
//!
//! Handles all incoming HTTP requests for creating and redirecting short links.

use crate::{
    database::{code_exists, delete_link, get_link, insert_link},
    models::{ApiError, AppState, ShortenRequest, ShortenResponse},
    utils::{generate_code, now_unix, parse_ttl, validate_code, validate_url},
};
use axum::{
    extract::{Path, State},
    response::{Json, Redirect},
};
use tracing::info;

/// POST /shorten - Creates a new short link
///
/// # Request Body
/// ```json
/// {
///   "url": "https://example.com",
///   "code": "optional_custom_code",
///   "ttl": "3d"
/// }
/// ```
///
/// # Response (200 OK)
/// ```json
/// {
///   "code": "abc123",
///   "short_url": "https://cutl.my.id/abc123",
///   "expires_at": 1760000000
/// }
/// ```
///
/// # Errors
/// - 400: Invalid URL, code, or TTL
/// - 401: Invalid or missing auth token
/// - 409: Code already exists
/// - 500: Internal server error
pub async fn shorten(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<ShortenRequest>,
) -> Result<Json<ShortenResponse>, ApiError> {
    // Validate auth token if configured
    if let Some(ref token) = state.auth_token {
        let auth_header = headers
            .get("authorization")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("");

        if !auth_header.starts_with("Bearer ") || auth_header[7..] != *token {
            return Err(ApiError::unauthorized(
                "Invalid or missing authorization token",
            ));
        }
    }

    // Validate URL
    validate_url(&req.url).map_err(|e| ApiError::bad_request(format!("Invalid URL: {}", e)))?;

    // Parse TTL or use default (7 days)
    let ttl_seconds = if let Some(ref ttl_str) = req.ttl {
        parse_ttl(ttl_str).map_err(|e| ApiError::bad_request(format!("Invalid TTL: {}", e)))?
    } else {
        // Default TTL: 7 days
        7 * 24 * 60 * 60
    };

    // Get or generate short code
    let code = if let Some(custom_code) = req.code {
        // Validate custom code format
        validate_code(&custom_code)
            .map_err(|e| ApiError::bad_request(format!("Invalid code: {}", e)))?;

        // Check if code already exists
        let exists = code_exists(&state.db, &custom_code)
            .await
            .map_err(|e| ApiError::internal(format!("Database error: {}", e)))?;

        if exists {
            return Err(ApiError::conflict(format!(
                "Code '{}' already exists",
                custom_code
            )));
        }

        custom_code
    } else {
        // Generate unique random code
        generate_unique_code(&state.db).await?
    };

    // Calculate expiration timestamp
    let expires_at = now_unix() + ttl_seconds;

    // Insert into database
    insert_link(&state.db, &code, &req.url, expires_at, now_unix())
        .await
        .map_err(|e| ApiError::internal(format!("Failed to save link: {}", e)))?;

    // Build response
    let short_url = format!("{}/{}", state.base_url.trim_end_matches('/'), code);
    info!("Created short link: {} -> {}", short_url, req.url);

    Ok(Json(ShortenResponse {
        code,
        short_url,
        expires_at,
    }))
}

/// GET /{code} - Redirects to the original URL
///
/// # Behavior
/// - Returns HTTP 302 redirect to the original URL
/// - Returns 404 if the link doesn't exist or has expired
///
/// # Errors
/// - 404: Link not found or expired
/// - 500: Internal server error
pub async fn redirect(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<Redirect, ApiError> {
    // Validate code format (basic check)
    if code.is_empty() || code.len() > 32 {
        return Err(ApiError::not_found("Short link not found"));
    }

    // Look up the link
    let link = get_link(&state.db, &code)
        .await
        .map_err(|e| ApiError::internal(format!("Database error: {}", e)))?;

    match link {
        Some(link) => {
            // Check if expired
            let now = now_unix();
            if now > link.expires_at {
                // Delete expired link
                delete_link(&state.db, &code).await.ok();

                return Err(ApiError::not_found("Short link has expired"));
            }

            info!("Redirecting {} to {}", code, link.original_url);
            Ok(Redirect::permanent(&link.original_url))
        }
        None => Err(ApiError::not_found("Short link not found")),
    }
}

/// Generates a unique code that doesn't exist in the database
///
/// Will attempt up to 10 times to generate a unique random code.
async fn generate_unique_code(db: &sqlx::Pool<sqlx::Sqlite>) -> Result<String, ApiError> {
    const MAX_ATTEMPTS: usize = 10;

    for _ in 0..MAX_ATTEMPTS {
        let code = generate_code();

        // Check if code already exists
        let exists = code_exists(db, &code)
            .await
            .map_err(|e| ApiError::internal(format!("Database error: {}", e)))?;

        if !exists {
            return Ok(code);
        }
    }

    Err(ApiError::internal(
        "Failed to generate unique code after multiple attempts",
    ))
}

/// POST /api/shorten - Creates short link without auth (for web UI)
///
/// Same logic as shorten() but without authentication check.
/// Rate limiting is applied via middleware.
///
/// # Request Body
/// ```json
/// {
///   "url": "https://example.com",
///   "code": "optional_custom_code",
///   "ttl": "3d"
/// }
/// ```
///
/// # Response (200 OK)
/// ```json
/// {
///   "code": "abc123",
///   "short_url": "https://cutl.my.id/abc123",
///   "expires_at": 1760000000
/// }
/// ```
///
/// # Errors
/// - 400: Invalid URL, code, or TTL
/// - 409: Code already exists
/// - 429: Rate limit exceeded
/// - 500: Internal server error
pub async fn shorten_noauth(
    State(state): State<AppState>,
    Json(req): Json<ShortenRequest>,
) -> Result<Json<ShortenResponse>, ApiError> {
    // NO auth check - this endpoint is for public web UI use
    // Rate limiting still applies via middleware

    // Validate URL
    validate_url(&req.url).map_err(|e| ApiError::bad_request(format!("Invalid URL: {}", e)))?;

    // Parse TTL or use default (7 days)
    let ttl_seconds = if let Some(ref ttl_str) = req.ttl {
        parse_ttl(ttl_str).map_err(|e| ApiError::bad_request(format!("Invalid TTL: {}", e)))?
    } else {
        // Default TTL: 7 days
        7 * 24 * 60 * 60
    };

    // Get or generate short code
    let code = if let Some(custom_code) = req.code {
        // Validate custom code format
        validate_code(&custom_code)
            .map_err(|e| ApiError::bad_request(format!("Invalid code: {}", e)))?;

        // Check if code already exists
        let exists = code_exists(&state.db, &custom_code)
            .await
            .map_err(|e| ApiError::internal(format!("Database error: {}", e)))?;

        if exists {
            return Err(ApiError::conflict(format!(
                "Code '{}' already exists",
                custom_code
            )));
        }

        custom_code
    } else {
        // Generate unique random code
        generate_unique_code(&state.db).await?
    };

    // Calculate expiration timestamp
    let expires_at = now_unix() + ttl_seconds;

    // Insert into database
    insert_link(&state.db, &code, &req.url, expires_at, now_unix())
        .await
        .map_err(|e| ApiError::internal(format!("Failed to save link: {}", e)))?;

    // Build response
    let short_url = format!("{}/{}", state.base_url.trim_end_matches('/'), code);
    info!("Created short link: {} -> {}", short_url, req.url);

    Ok(Json(ShortenResponse {
        code,
        short_url,
        expires_at,
    }))
}
