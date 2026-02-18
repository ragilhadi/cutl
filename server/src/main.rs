//! cutl Server - Self-hosted URL Shortener
//!
//! A simple HTTP API for shortening URLs with custom codes and TTL support.
//!
//! # Features
//! - Create short links with custom or auto-generated codes
//! - Set expiration times (TTL)
//! - Redirect short links to original URLs
//! - Automatic cleanup of expired links

mod config;
mod database;
mod handlers;
mod middleware;
mod models;
mod utils;

use crate::{
    config::Config, database::delete_expired_links, middleware::create_rate_limiter,
    models::AppState, utils::now_unix,
};
use axum::{
    routing::{get, post},
    Router,
};
use std::time::Duration;
use tokio::time::interval;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "cutl_server=debug,tower_http=debug,axum=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load .env file if present
    dotenv::dotenv().ok();

    // Load configuration from environment
    let config = Config::from_env()?;

    info!("Starting cutl server");
    info!("Database: {}", config.database_url);
    info!("Base URL: {}", config.base_url);
    info!("Bind address: {}", config.bind_address);
    info!(
        "Rate limit: {} requests/minute (burst: {})",
        config.rate_limit, config.rate_limit_burst
    );

    // Create database connection pool
    let db = database::create_pool(&config.database_url).await?;

    // Run migrations automatically
    database::run_migrations(&db).await?;

    // Initialize GeoIP reader if configured
    let geoip =
        config.geoip_db_path.as_ref().and_then(|path| {
            match maxminddb::Reader::open_readfile(path) {
                Ok(r) => {
                    info!("GeoIP database loaded from {}", path);
                    Some(std::sync::Arc::new(r))
                }
                Err(e) => {
                    tracing::warn!("Could not load GeoIP database: {}", e);
                    None
                }
            }
        });

    // Create application state
    let state = AppState {
        db,
        base_url: config.base_url,
        auth_token: config.auth_token,
        geoip,
    };

    // Spawn background task for cleanup
    let cleanup_state = state.clone();
    tokio::spawn(async move {
        cleanup_task(cleanup_state).await;
    });

    // Create rate limiter
    let rate_limiter = create_rate_limiter(config.rate_limit, config.rate_limit_burst);

    // Configure CORS to allow frontend requests
    let cors = CorsLayer::permissive();

    // Build the router
    let app = Router::new()
        // Rate-limited routes for shortening
        .route("/shorten", post(handlers::shorten))
        .route("/api/shorten", post(handlers::shorten_noauth))
        .layer(rate_limiter)
        // Public redirect and analytics (no rate limit)
        .route("/{code}", get(handlers::redirect))
        .route("/analytics/{code}", get(handlers::analytics))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Start the server
    let listener = tokio::net::TcpListener::bind(&config.bind_address).await?;
    info!("Server listening on {}", config.bind_address);
    axum::serve(listener, app).await?;

    Ok(())
}

/// Background task that periodically deletes expired links
///
/// Runs every 60 seconds and cleans up any links that have expired.
async fn cleanup_task(state: AppState) {
    let mut timer = interval(Duration::from_secs(60));

    loop {
        timer.tick().await;

        let now = now_unix();

        match delete_expired_links(&state.db, now).await {
            Ok(count) => {
                if count > 0 {
                    info!("Cleaned up {} expired links", count);
                }
            }
            Err(e) => {
                tracing::error!("Failed to cleanup expired links: {}", e);
            }
        }
    }
}
