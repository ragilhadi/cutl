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
