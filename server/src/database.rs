//! Database operations for the cutl server
//!
//! Handles all SQLite database operations including migrations, CRUD operations,
//! and cleanup of expired links.

use crate::models::{Link, VisitRow};
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

    // Create the visits table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS visits (
            id         INTEGER PRIMARY KEY AUTOINCREMENT,
            code       TEXT    NOT NULL REFERENCES links(code) ON DELETE CASCADE,
            visited_at INTEGER NOT NULL,
            ip         TEXT,
            country    TEXT,
            city       TEXT,
            user_agent TEXT,
            referer    TEXT
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_visits_code ON visits(code)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_visits_visited_at ON visits(visited_at)")
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

/// Records a single visit for a short code.
#[allow(clippy::too_many_arguments)]
pub async fn insert_visit(
    pool: &Pool<Sqlite>,
    code: &str,
    visited_at: i64,
    ip: Option<&str>,
    country: Option<&str>,
    city: Option<&str>,
    user_agent: Option<&str>,
    referer: Option<&str>,
) -> Result<()> {
    sqlx::query(
        "INSERT INTO visits (code, visited_at, ip, country, city, user_agent, referer) VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(code)
    .bind(visited_at)
    .bind(ip)
    .bind(country)
    .bind(city)
    .bind(user_agent)
    .bind(referer)
    .execute(pool)
    .await?;

    Ok(())
}

/// Returns total visit count for `code`.
pub async fn count_visits(pool: &Pool<Sqlite>, code: &str) -> Result<i64> {
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM visits WHERE code = ?")
        .bind(code)
        .fetch_one(pool)
        .await?;

    Ok(count)
}

/// Returns visit counts grouped by country, ordered by count DESC.
pub async fn visits_by_country(
    pool: &Pool<Sqlite>,
    code: &str,
) -> Result<Vec<(Option<String>, i64)>> {
    let rows = sqlx::query_as::<_, (Option<String>, i64)>(
        "SELECT country, COUNT(*) as count FROM visits WHERE code = ? GROUP BY country ORDER BY count DESC",
    )
    .bind(code)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/// Returns visit counts grouped by referer, ordered by count DESC.
pub async fn visits_by_referer(
    pool: &Pool<Sqlite>,
    code: &str,
) -> Result<Vec<(Option<String>, i64)>> {
    let rows = sqlx::query_as::<_, (Option<String>, i64)>(
        "SELECT referer, COUNT(*) as count FROM visits WHERE code = ? GROUP BY referer ORDER BY count DESC",
    )
    .bind(code)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/// Returns daily visit counts for the last 30 days, newest first.
pub async fn visits_daily(pool: &Pool<Sqlite>, code: &str) -> Result<Vec<(String, i64)>> {
    let rows = sqlx::query_as::<_, (String, i64)>(
        r#"SELECT strftime('%Y-%m-%d', datetime(visited_at, 'unixepoch')) as date,
                  COUNT(*) as count
           FROM visits
           WHERE code = ?
             AND visited_at >= strftime('%s', 'now', '-30 days')
           GROUP BY date
           ORDER BY date DESC"#,
    )
    .bind(code)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/// Returns the last 20 individual visit rows for `code`, newest first.
pub async fn recent_visits(pool: &Pool<Sqlite>, code: &str) -> Result<Vec<VisitRow>> {
    let rows = sqlx::query_as::<_, (i64, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>)>(
        "SELECT visited_at, ip, country, city, user_agent, referer FROM visits WHERE code = ? ORDER BY visited_at DESC LIMIT 20",
    )
    .bind(code)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(
            |(visited_at, ip, country, city, user_agent, referer)| VisitRow {
                visited_at,
                ip,
                country,
                city,
                user_agent,
                referer,
            },
        )
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_db() -> Pool<Sqlite> {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        run_migrations(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn test_insert_and_count_visits() {
        let pool = setup_db().await;
        insert_link(&pool, "abc", "https://example.com", 9999999999, 1000000000)
            .await
            .unwrap();

        insert_visit(
            &pool,
            "abc",
            1000000001,
            Some("1.2.3.4"),
            Some("US"),
            Some("New York"),
            Some("Mozilla/5.0"),
            None,
        )
        .await
        .unwrap();
        insert_visit(
            &pool,
            "abc",
            1000000002,
            Some("5.6.7.8"),
            Some("ID"),
            Some("Jakarta"),
            None,
            Some("https://twitter.com/"),
        )
        .await
        .unwrap();
        insert_visit(&pool, "abc", 1000000003, None, None, None, None, None)
            .await
            .unwrap();

        let count = count_visits(&pool, "abc").await.unwrap();
        assert_eq!(count, 3);
    }

    #[tokio::test]
    async fn test_visits_by_country() {
        let pool = setup_db().await;
        insert_link(&pool, "xyz", "https://example.com", 9999999999, 1000000000)
            .await
            .unwrap();

        insert_visit(&pool, "xyz", 1000000001, None, Some("ID"), None, None, None)
            .await
            .unwrap();
        insert_visit(&pool, "xyz", 1000000002, None, Some("ID"), None, None, None)
            .await
            .unwrap();
        insert_visit(&pool, "xyz", 1000000003, None, Some("US"), None, None, None)
            .await
            .unwrap();

        let rows = visits_by_country(&pool, "xyz").await.unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].0, Some("ID".to_string()));
        assert_eq!(rows[0].1, 2);
        assert_eq!(rows[1].0, Some("US".to_string()));
        assert_eq!(rows[1].1, 1);
    }
}
