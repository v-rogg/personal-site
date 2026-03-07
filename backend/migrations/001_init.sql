-- Enable WAL mode for better concurrent performance
PRAGMA journal_mode = WAL;
PRAGMA synchronous = NORMAL;
PRAGMA foreign_keys = ON;

-- Users table
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT,
    ts_created INTEGER NOT NULL,
    CONSTRAINT email_unique UNIQUE(email)
);

CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);

-- Sessions table (with geo data)
CREATE TABLE IF NOT EXISTS sessions (
    id TEXT PRIMARY KEY,
    user_id INTEGER,
    country_code TEXT,
    region TEXT,
    ts_created INTEGER NOT NULL,
    ts_last_seen INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_sessions_user ON sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_sessions_last_seen ON sessions(ts_last_seen);
CREATE INDEX IF NOT EXISTS idx_sessions_country ON sessions(country_code);

-- Signatures table
CREATE TABLE IF NOT EXISTS signatures (
    id TEXT PRIMARY KEY,
    session_id TEXT,
    name TEXT NOT NULL,
    signature TEXT NOT NULL,
    approved INTEGER NOT NULL DEFAULT 0,
    ts_created INTEGER NOT NULL,
    ts_modified INTEGER,
    FOREIGN KEY (session_id) REFERENCES sessions(id)
);

CREATE INDEX IF NOT EXISTS idx_signatures_session ON signatures(session_id);
CREATE INDEX IF NOT EXISTS idx_signatures_approved ON signatures(approved);
CREATE INDEX IF NOT EXISTS idx_signatures_created ON signatures(ts_created);

-- Events table
CREATE TABLE IF NOT EXISTS events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT NOT NULL,
    event_name TEXT NOT NULL,
    timestamp INTEGER NOT NULL,
    properties TEXT,
    FOREIGN KEY (session_id) REFERENCES sessions(id)
);

CREATE INDEX IF NOT EXISTS idx_events_session ON events(session_id);
CREATE INDEX IF NOT EXISTS idx_events_name ON events(event_name);
CREATE INDEX IF NOT EXISTS idx_events_timestamp ON events(timestamp);
CREATE INDEX IF NOT EXISTS idx_events_name_ts ON events(event_name, timestamp);

-- Page views table (engagement tracking)
CREATE TABLE IF NOT EXISTS page_views (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT NOT NULL,
    path TEXT NOT NULL,
    referrer TEXT,
    ts_start INTEGER NOT NULL,
    ts_end INTEGER,
    duration_seconds INTEGER,
    max_scroll_percent INTEGER DEFAULT 0,
    FOREIGN KEY (session_id) REFERENCES sessions(id)
);

CREATE INDEX IF NOT EXISTS idx_page_views_session ON page_views(session_id);
CREATE INDEX IF NOT EXISTS idx_page_views_path ON page_views(path);
CREATE INDEX IF NOT EXISTS idx_page_views_start ON page_views(ts_start);
