# AGENTS.md - Agent Development Guide

## Project Overview

This is a Rust workspace for `cutl`, a self-hosted URL shortener with a CLI client and HTTP API server.

- **Workspace members**: `server`, `cli`
- **Minimum Rust version**: 1.83

## Build, Lint, and Test Commands

### Building
```bash
cargo build --release              # Build release binaries
cargo build -p cutl                # Build CLI only
cargo build -p cutl-server         # Build server only
make build                          # Build Docker image
```

### Testing
```bash
cargo test --workspace              # Run all tests
cargo test -p cutl                  # Run CLI tests only
cargo test -p cutl-server           # Run server tests only
make test                           # Run all tests (via Makefile)
make test-cli                       # Run CLI tests (via Makefile)
make test-server                    # Run server tests (via Makefile)
```

#### Running a Single Test
```bash
# Run a specific test function
cargo test test_function_name --workspace

# Run a test in a specific module
cargo test module_name::test_name --manifest-path cli/Cargo.toml

# Run tests matching a pattern
cargo test validate --workspace

# Run tests with output
cargo test -- --nocapture --test-threads=1 test_name
```

### Linting and Formatting
```bash
cargo fmt --all                     # Format all code
cargo fmt --all -- --check          # Check formatting without modifying
cargo clippy --workspace --all-targets --all-features -- -D warnings  # Lint with warnings as errors
```

## Code Style Guidelines

### Imports
- Group external imports first, then internal crate imports
- Use `use crate::` for internal modules
- Keep imports sorted alphabetically where sensible

```rust
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::config::Config;
use crate::models::AppState;
```

### Formatting
- Use `cargo fmt` (4 spaces, 100 char line limit)
- No trailing commas in single-line struct/enum definitions
- Trailing commas in multi-line definitions

### Types
- Use `anyhow::Result<T>` for general error handling
- Custom error types for API responses (see `models.rs`)
- Domain models as plain structs with `#[derive(Debug, Clone)]`
- Request/Response types with `#[derive(Deserialize)]`/`#[derive(Serialize)]`

```rust
#[derive(Debug, Clone)]
pub struct AppState { ... }

#[derive(Debug, Deserialize)]
pub struct ShortenRequest { ... }
```

### Naming Conventions
- Functions and variables: `snake_case`
- Structs and enums: `PascalCase`
- Constants: `SCREAMING_SNAKE_CASE`
- Private fields: `snake_case` (no underscore prefix)
- Use `#[allow(dead_code)]` for intentionally unused documented fields

```rust
pub const MAX_TTL_SECONDS: i64 = 30 * 24 * 60 * 60;
pub fn generate_code() -> String { ... }
pub struct ShortenRequest { pub url: String, ... }
```

### Error Handling
- Use `anyhow::Result<T>` as return type for fallible functions
- Use `anyhow::bail!("message")` for early returns with errors
- Use `.context("description")` to add error context
- For API handlers, use `ApiError` with status codes

```rust
pub fn parse_ttl(ttl: &str) -> anyhow::Result<i64> {
    let num: i64 = num_str.parse()
        .map_err(|_| anyhow::anyhow!("Invalid TTL number"))?;
    if seconds < MIN_TTL_SECONDS {
        bail!("TTL must be at least {} seconds", MIN_TTL_SECONDS);
    }
    Ok(seconds)
}
```

### Documentation
- Module-level: `//!` comment at top of file
- Public functions: `///` doc comments
- Include `# Arguments`, `# Returns`, `# Errors` sections where applicable

```rust
//! Database operations for the cutl server

/// Creates a new database connection pool
///
/// # Arguments
/// * `database_url` - SQLite connection string (e.g., "sqlite:cutl.db")
pub async fn create_pool(database_url: &str) -> Result<Pool<Sqlite>> { ... }
```

### Testing
- All functions should have corresponding unit tests
- Tests in `#[cfg(test)] mod tests` blocks at end of files
- Test naming: `test_<function_name>` or descriptive like `test_validate_url_valid`
- Use `assert_eq!`, `assert!`, `assert!()` for assertions

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ttl_valid() {
        assert_eq!(parse_ttl("5m").unwrap(), 300);
    }

    #[test]
    fn test_parse_ttl_invalid() {
        assert!(parse_ttl("invalid").is_err());
    }
}
```

### Configuration
- Environment variables for configuration (e.g., `DATABASE_URL`, `BASE_URL`)
- Default values provided via `unwrap_or_else`
- `.env` file support via `dotenv` crate (for local dev)

### Async/Await
- `#[tokio::main]` for async entry points
- `async fn` for async functions
- Use `.await` correctly, prefer chaining over intermediate variables

### Database
- SQLx with SQLite
- Use `?` operator for propagating errors
- Use `sqlx::query!` or `sqlx::query_as!` for type-safe queries

### HTTP Server (Axum)
- Router with `.route("/path", handler)`
- Extractors: `State`, `Path`, `Json`
- Return `Result<Json<T>, ApiError>` for JSON responses
- Return `Result<Redirect, ApiError>` for redirects

### HTTP Client (Reqwest)
- Timeout configured to 30 seconds
- Use `.bearer_auth()` for authentication
- Parse error responses before returning

### Constants and Magic Numbers
- Define constants for magic numbers (e.g., `MIN_TTL_SECONDS`, `MAX_TTL_SECONDS`)
- Use `lazy_static` for regex patterns that shouldn't be recompiled

## CI/CD Pipeline

The project uses GitHub Actions for automated checks:

- **Format check**: `cargo fmt --all -- --check`
- **Clippy**: `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- **Tests**: `cargo test --workspace --verbose`
- **Build**: `cargo build --workspace --verbose`

All checks must pass before merging.
