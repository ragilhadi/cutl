//! HTTP request handlers for the cutl server
//!
//! Handles all incoming HTTP requests for creating and redirecting short links.

use crate::{
    database::{
        code_exists, count_visits, delete_link, get_link, insert_link, insert_visit, recent_visits,
        visits_by_country, visits_by_referer, visits_daily,
    },
    models::{
        AnalyticsResponse, ApiError, AppState, CountStat, DailyStat, ShortenRequest,
        ShortenResponse,
    },
    utils::{
        extract_client_ip, generate_code, now_unix, parse_ttl, resolve_geo, validate_code,
        validate_url,
    },
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
    headers: axum::http::HeaderMap,
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

            // Record visit (best-effort, don't fail redirect on analytics error)
            let ip = extract_client_ip(&headers);
            let (country, city) = if let (Some(ref r), Some(ref ip_str)) = (&state.geoip, &ip) {
                resolve_geo(r, ip_str)
            } else {
                (None, None)
            };
            let ua = headers
                .get("user-agent")
                .and_then(|v| v.to_str().ok())
                .map(str::to_owned);
            let ref_ = headers
                .get("referer")
                .and_then(|v| v.to_str().ok())
                .map(str::to_owned);

            insert_visit(
                &state.db,
                &code,
                now_unix(),
                ip.as_deref(),
                country.as_deref(),
                city.as_deref(),
                ua.as_deref(),
                ref_.as_deref(),
            )
            .await
            .ok(); // swallow errors — redirect still completes

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

/// GET /analytics/{code} – Returns visit statistics for a short link
///
/// # Errors
/// - 401: Missing/invalid token (when auth is enabled)
/// - 404: Code not found or expired
pub async fn analytics(
    State(state): State<AppState>,
    Path(code): Path<String>,
    headers: axum::http::HeaderMap,
) -> Result<Json<AnalyticsResponse>, ApiError> {
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

    // Look up the link
    let link = get_link(&state.db, &code)
        .await
        .map_err(|e| ApiError::internal(format!("Database error: {}", e)))?
        .ok_or_else(|| ApiError::not_found("Short link not found"))?;

    // Check if expired
    if now_unix() > link.expires_at {
        return Err(ApiError::not_found("Short link has expired"));
    }

    let total_visits = count_visits(&state.db, &code)
        .await
        .map_err(|e| ApiError::internal(format!("Database error: {}", e)))?;

    let countries = visits_by_country(&state.db, &code)
        .await
        .map_err(|e| ApiError::internal(format!("Database error: {}", e)))?
        .into_iter()
        .map(|(value, count)| CountStat { value, count })
        .collect();

    let referers = visits_by_referer(&state.db, &code)
        .await
        .map_err(|e| ApiError::internal(format!("Database error: {}", e)))?
        .into_iter()
        .map(|(value, count)| CountStat { value, count })
        .collect();

    let daily = visits_daily(&state.db, &code)
        .await
        .map_err(|e| ApiError::internal(format!("Database error: {}", e)))?
        .into_iter()
        .map(|(date, count)| DailyStat { date, count })
        .collect();

    let recent = recent_visits(&state.db, &code)
        .await
        .map_err(|e| ApiError::internal(format!("Database error: {}", e)))?;

    Ok(Json(AnalyticsResponse {
        code: link.code,
        original_url: link.original_url,
        created_at: link.created_at,
        expires_at: link.expires_at,
        total_visits,
        countries,
        referers,
        daily,
        recent_visits: recent,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::get,
        Router,
    };
    use sqlx::sqlite::SqlitePool;
    use tower::ServiceExt;

    async fn setup_app() -> Router {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        crate::database::run_migrations(&pool).await.unwrap();

        let state = AppState {
            db: pool,
            base_url: "http://localhost:3000".to_string(),
            auth_token: None,
            geoip: None,
        };

        Router::new()
            .route("/{code}", get(redirect))
            .route("/analytics/{code}", get(analytics))
            .with_state(state)
    }

    #[tokio::test]
    async fn test_analytics_not_found() {
        let app = setup_app().await;

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/analytics/noexist")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_analytics_returns_counts() {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        crate::database::run_migrations(&pool).await.unwrap();

        // Create a link that expires far in the future
        crate::database::insert_link(
            &pool,
            "testcode",
            "https://example.com",
            9999999999,
            1000000000,
        )
        .await
        .unwrap();

        let state = AppState {
            db: pool,
            base_url: "http://localhost:3000".to_string(),
            auth_token: None,
            geoip: None,
        };

        let app = Router::new()
            .route("/{code}", get(redirect))
            .route("/analytics/{code}", get(analytics))
            .with_state(state);

        // Trigger two redirects to record visits
        app.clone()
            .oneshot(
                Request::builder()
                    .uri("/testcode")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        app.clone()
            .oneshot(
                Request::builder()
                    .uri("/testcode")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Call analytics endpoint
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/analytics/testcode")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["total_visits"], 2);
    }
}
