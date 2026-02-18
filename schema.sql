-- cutl URL Shortener Database Schema
-- SQLite database for storing shortened URLs

CREATE TABLE IF NOT EXISTS links (
    -- Short code (primary key)
    -- Must match: ^[a-zA-Z0-9_-]{1,32}$
    code TEXT PRIMARY KEY,

    -- Original URL to redirect to
    original_url TEXT NOT NULL,

    -- Expiration timestamp (UNIX timestamp in seconds)
    expires_at INTEGER NOT NULL,

    -- Creation timestamp (UNIX timestamp in seconds)
    created_at INTEGER NOT NULL
);

-- Index for faster expiration-based cleanup
CREATE INDEX IF NOT EXISTS idx_links_expires_at ON links(expires_at);

-- Index for faster lookups (though primary key is already indexed)
CREATE INDEX IF NOT EXISTS idx_links_code ON links(code);

-- Visit tracking for analytics
CREATE TABLE IF NOT EXISTS visits (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,

    -- Foreign key to links table; cascades on delete
    code       TEXT    NOT NULL REFERENCES links(code) ON DELETE CASCADE,

    -- UNIX timestamp (seconds) when the visit occurred
    visited_at INTEGER NOT NULL,

    -- Raw IP address (IPv4 or IPv6); may be NULL if not determinable
    ip         TEXT,

    -- ISO 3166-1 alpha-2 country code (e.g. "ID", "US")
    -- Populated only when GEOIP_DB_PATH is configured
    country    TEXT,

    -- City name, best-effort; NULL when GeoIP is not configured
    city       TEXT,

    -- Full User-Agent header value; NULL if not sent
    user_agent TEXT,

    -- Referer header value; NULL if not sent
    referer    TEXT
);

-- Index for fast per-code analytics queries
CREATE INDEX IF NOT EXISTS idx_visits_code ON visits(code);

-- Index for time-range queries (e.g. daily aggregations)
CREATE INDEX IF NOT EXISTS idx_visits_visited_at ON visits(visited_at);
