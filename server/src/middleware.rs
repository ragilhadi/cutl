//! Rate limiting middleware

use axum::body::Body;
use governor::clock::QuantaInstant;
use governor::middleware::NoOpMiddleware;
use tower_governor::{
    governor::GovernorConfigBuilder, key_extractor::SmartIpKeyExtractor, GovernorLayer,
};

/// Creates the rate limiter middleware layer
///
/// Uses SmartIpKeyExtractor which automatically extracts the client IP from:
/// - X-Forwarded-For header (first IP)
/// - X-Real-IP header
/// - Forwarded header
/// - Connection IP (fallback)
///
/// # Arguments
/// * `rate_limit` - Maximum requests per minute
/// * `burst_size` - How many requests can happen in quick succession
///
/// # Returns
/// A GovernorLayer that can be used with `.layer()`
pub fn create_rate_limiter(
    rate_limit: u32,
    burst_size: u32,
) -> GovernorLayer<SmartIpKeyExtractor, NoOpMiddleware<QuantaInstant>, Body> {
    // Build governor configuration
    // Use per_second to calculate the rate: 60 seconds / rate_limit
    let seconds_per_request = 60u64 / rate_limit as u64;

    let config = GovernorConfigBuilder::default()
        .key_extractor(SmartIpKeyExtractor)
        .per_second(seconds_per_request)
        .burst_size(burst_size)
        .finish()
        .unwrap();

    GovernorLayer::new(config)
}
