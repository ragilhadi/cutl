# AGENTS.md - Agent Development Guide

## Project Overview

Rust workspace for `cutl`, a self-hosted URL shortener with three components:

- **`server/`** — Axum HTTP API server with SQLite storage (`cutl-server`)
- **`cli/`** — CLI client (`cutl`)
- **`frontend/`** — Vanilla TypeScript + Vite SPA (no framework)
- **Minimum Rust version**: 1.83

## Architecture

```
CLI (cutl) ──HTTP──► Server (cutl-server) ──SQLite──► cutl.db
                           │
Frontend (Vite/TS) ──HTTP──┘
```

### Server Layers ([server/src/](server/src/))

| File | Responsibility |
|---|---|
| `main.rs` | Router setup, middleware, background cleanup task |
| `handlers.rs` | Route handlers: `shorten`, `shorten_noauth`, `redirect` |
| `models.rs` | `AppState`, `ApiError`, request/response types |
| `database.rs` | SQLx query functions, `run_migrations()` |
| `middleware.rs` | Rate limiting via `tower_governor` |
| `utils.rs` | `generate_code()`, `parse_ttl()`, `validate_url()` |
| `config.rs` | Env var loading with defaults |

### API Routes

| Method | Path | Auth | Rate Limited |
|---|---|---|---|
| `POST` | `/shorten` | Bearer token (if `AUTH_TOKEN` set) | Yes |
| `POST` | `/api/shorten` | None | Yes |
| `GET` | `/{code}` | None | No |

Request body: `{ "url": "...", "code": "optional", "ttl": "3d" }`
Response: `{ "code": "abc123", "short_url": "https://cutl.my.id/abc123", "expires_at": 1760000000 }`

### Database Schema ([schema.sql](schema.sql))

```sql
CREATE TABLE links (
    code TEXT PRIMARY KEY CHECK(code REGEXP '^[a-zA-Z0-9_-]{1,32}$'),
    original_url TEXT NOT NULL,
    expires_at INTEGER NOT NULL,  -- UNIX timestamp
    created_at INTEGER NOT NULL   -- UNIX timestamp
);
```

Migrations run automatically at startup via `database::run_migrations()` — no external tool needed.
A `tokio::spawn` background task purges expired rows every 60 seconds.

### Key Types ([server/src/models.rs](server/src/models.rs))

```rust
pub struct AppState {
    pub db: sqlx::Pool<sqlx::Sqlite>,
    pub base_url: String,
    pub auth_token: Option<String>,
}

pub struct ApiError { pub status: StatusCode, pub message: String }
// Constructors: ApiError::bad_request, ::unauthorized, ::not_found, ::conflict, ::internal
// Implements IntoResponse → {"error": "message"} and From<anyhow::Error>
```

## Build, Lint, and Test Commands

```bash
cargo build --release                                                    # Release binaries
cargo build -p cutl                                                      # CLI only
cargo build -p cutl-server                                               # Server only
cargo fmt --all                                                          # Format
cargo fmt --all -- --check                                               # Check formatting
cargo clippy --workspace --all-targets --all-features -- -D warnings     # Lint (CI-strict)
cargo test --workspace                                                   # All tests
cargo test -p cutl                                                       # CLI tests only
cargo test -p cutl-server                                                # Server tests only
cargo test test_function_name --workspace                                # Single test
cargo test -- --nocapture --test-threads=1 test_name                    # With output
make build                                                               # Docker image
```

## Code Style Guidelines

- `cargo fmt`: 4 spaces, 100-char line limit
- Trailing commas in multi-line; none in single-line definitions
- External imports first, then `use crate::`, sorted alphabetically
- `anyhow::Result<T>` throughout; `ApiError` only at the HTTP response boundary
- Use `bail!("msg")` for early error returns, `.context("desc")` to add context
- `lazy_static!` for regex patterns (see `utils.rs` for `CODE_REGEX`)
- Constants for all magic numbers (see `MIN_TTL_SECONDS`, `MAX_TTL_SECONDS` in `utils.rs`)

### Documentation

```rust
//! Module-level doc with //!
/// Public fn doc with ///
/// # Arguments / # Returns / # Errors
```

### Testing

Tests go in `#[cfg(test)] mod tests` at the end of each file. Naming: `test_<function>` or descriptive (`test_validate_url_valid`).

## Configuration

### Server env vars ([server/src/config.rs](server/src/config.rs))

| Var | Default |
|---|---|
| `DATABASE_URL` | `sqlite:cutl.db` |
| `BASE_URL` | `http://localhost:3000` |
| `BIND_ADDRESS` | `0.0.0.0:3000` |
| `AUTH_TOKEN` | _(none — disables auth on `/shorten`)_ |
| `RATE_LIMIT` | `10` (requests/min) |
| `RATE_LIMIT_BURST` | `2` |

`.env` loaded via `dotenv` at startup.

### CLI env vars ([cli/src/config.rs](cli/src/config.rs))

| Var | Default |
|---|---|
| `CUTL_SERVER` | `https://cutl.my.id` |
| `CUTL_TOKEN` | _(none)_ |

## CLI Usage

```
cutl <URL> [--code/-c <code>] [--ttl/-t <ttl>] [--server/-s <url>]
```

TTL format: `5m`, `1h`, `3d`, `30d` — min 5m, max 30d. Code format: `[a-zA-Z0-9_-]{1,32}`.

## Integration Points

- **Auth**: CLI sends `$CUTL_TOKEN` as `Authorization: Bearer <token>`; server validates against `$AUTH_TOKEN`
- **Rate limiting**: `tower_governor` on `/shorten` and `/api/shorten`; IP extracted from `X-Forwarded-For`, `X-Real-IP`, `Forwarded`, or direct connection
- **CORS**: `CorsLayer::permissive()` globally (for frontend)
- **Frontend**: Vite dev server on port `3234`; calls `/api/shorten` (no auth required)

## CI/CD Pipeline

GitHub Actions (`.github/workflows/test.yml`) on push/PR to `master`:
1. `cargo fmt --all -- --check`
2. `cargo clippy --workspace --all-targets --all-features -- -D warnings`
3. `cargo test -p cutl --verbose`
4. `cargo test -p cutl-server --verbose`
5. `cargo test --workspace --verbose`
6. Build check: `cargo build` for each crate and workspace

All checks must pass before merging.
