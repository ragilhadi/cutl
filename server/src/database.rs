//! Database operations for the cutl server
//!
//! Handles all SQLite database operations including migrations, CRUD operations,
//! and cleanup of expired links.

use crate::models::Link;
use anyhow::Result;
use sqlx::{sqlite::SqlitePool, Pool, Sqlite};
use tracing::info;

/// Creates a new database connection pool
///
/// # Arguments
/// * `database_url` - SQLite connection string (e.g., "sqlite:cutl.db")
pub async fn create_pool(database_url: &str) -> Result<Pool<Sqlite>> {
    let pool = SqlitePool::connect(database_url).await?;
    Ok(pool)
}

/// Runs database migrations
///
/// Creates the links table and indexes if they don't exist.
pub async fn run_migrations(pool: &Pool<Sqlite>) -> Result<()> {
    // Create the links table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS links (
            code TEXT PRIMARY KEY,
            original_url TEXT NOT NULL,
            expires_at INTEGER NOT NULL,
            created_at INTEGER NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create index on expires_at for faster cleanup queries
    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_links_expires_at ON links(expires_at)
        "#,
    )
    .execute(pool)
    .await?;

    // Create index on code for faster lookups (redundant with primary key but explicit)
    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_links_code ON links(code)
        "#,
    )
    .execute(pool)
    .await?;

    info!("Database migrations completed");
    Ok(())
}

/// Checks if a short code already exists in the database
pub async fn code_exists(pool: &Pool<Sqlite>, code: &str) -> Result<bool> {
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM links WHERE code = ?")
        .bind(code)
        .fetch_one(pool)
        .await?;

    Ok(count > 0)
}

/// Inserts a new link into the database
pub async fn insert_link(
    pool: &Pool<Sqlite>,
    code: &str,
    original_url: &str,
    expires_at: i64,
    created_at: i64,
) -> Result<()> {
    sqlx::query(
        "INSERT INTO links (code, original_url, expires_at, created_at) VALUES (?, ?, ?, ?)",
    )
    .bind(code)
    .bind(original_url)
    .bind(expires_at)
    .bind(created_at)
    .execute(pool)
    .await?;

    Ok(())
}

/// Retrieves a link by its short code
///
/// Returns `None` if the code doesn't exist.
pub async fn get_link(pool: &Pool<Sqlite>, code: &str) -> Result<Option<Link>> {
    let result = sqlx::query_as::<_, (String, String, i64, i64)>(
        "SELECT code, original_url, expires_at, created_at FROM links WHERE code = ?",
    )
    .bind(code)
    .fetch_optional(pool)
    .await?;

    Ok(
        result.map(|(code, original_url, expires_at, created_at)| Link {
            code,
            original_url,
            expires_at,
            created_at,
        }),
    )
}

/// Deletes a link by its short code
pub async fn delete_link(pool: &Pool<Sqlite>, code: &str) -> Result<bool> {
    let result = sqlx::query("DELETE FROM links WHERE code = ?")
        .bind(code)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}

/// Deletes all expired links from the database
///
/// Returns the number of links deleted.
pub async fn delete_expired_links(pool: &Pool<Sqlite>, now: i64) -> Result<u64> {
    let result = sqlx::query("DELETE FROM links WHERE expires_at < ?")
        .bind(now)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}
