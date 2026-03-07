-- Migration: Schema updates
-- 1. Change signatures.approved to nullable boolean (NULL = not reviewed)
-- 2. Change users.id to TEXT (UUID)
-- 3. Change events.id to TEXT (UUID) and rename timestamp to ts
-- 4. Change page_views.id to TEXT (UUID)

-- SQLite doesn't support ALTER COLUMN, so we need to recreate tables

-- Disable foreign keys temporarily
PRAGMA foreign_keys = OFF;

-- ============================================
-- 1. Recreate users table with TEXT id
-- ============================================
CREATE TABLE users_new (
    id TEXT PRIMARY KEY,
    email TEXT,
    ts_created INTEGER NOT NULL,
    CONSTRAINT email_unique UNIQUE(email)
);

-- Migrate data (convert INTEGER id to UUID format)
INSERT INTO users_new (id, email, ts_created)
SELECT
    lower(hex(randomblob(4)) || '-' || hex(randomblob(2)) || '-4' || substr(hex(randomblob(2)),2) || '-' || substr('89ab',abs(random()) % 4 + 1, 1) || substr(hex(randomblob(2)),2) || '-' || hex(randomblob(6))),
    email,
    ts_created
FROM users;

DROP TABLE users;
ALTER TABLE users_new RENAME TO users;

CREATE INDEX idx_users_email ON users(email);

-- ============================================
-- 2. Recreate signatures table with nullable approved
-- ============================================
CREATE TABLE signatures_new (
    id TEXT PRIMARY KEY,
    session_id TEXT,
    name TEXT NOT NULL,
    signature TEXT NOT NULL,
    approved INTEGER DEFAULT NULL,
    ts_created INTEGER NOT NULL,
    ts_modified INTEGER,
    FOREIGN KEY (session_id) REFERENCES sessions(id)
);

-- Migrate data (convert 0 to NULL for not reviewed, keep 1 as approved)
INSERT INTO signatures_new (id, session_id, name, signature, approved, ts_created, ts_modified)
SELECT
    id,
    session_id,
    name,
    signature,
    CASE WHEN approved = 1 THEN 1 ELSE NULL END,
    ts_created,
    ts_modified
FROM signatures;

DROP TABLE signatures;
ALTER TABLE signatures_new RENAME TO signatures;

CREATE INDEX idx_signatures_session ON signatures(session_id);
CREATE INDEX idx_signatures_approved ON signatures(approved);
CREATE INDEX idx_signatures_created ON signatures(ts_created);

-- ============================================
-- 3. Recreate events table with TEXT id and ts column
-- ============================================
CREATE TABLE events_new (
    id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL,
    event_name TEXT NOT NULL,
    ts INTEGER NOT NULL,
    properties TEXT,
    FOREIGN KEY (session_id) REFERENCES sessions(id)
);

-- Migrate data with new UUIDs
INSERT INTO events_new (id, session_id, event_name, ts, properties)
SELECT
    lower(hex(randomblob(4)) || '-' || hex(randomblob(2)) || '-4' || substr(hex(randomblob(2)),2) || '-' || substr('89ab',abs(random()) % 4 + 1, 1) || substr(hex(randomblob(2)),2) || '-' || hex(randomblob(6))),
    session_id,
    event_name,
    timestamp,
    properties
FROM events;

DROP TABLE events;
ALTER TABLE events_new RENAME TO events;

CREATE INDEX idx_events_session ON events(session_id);
CREATE INDEX idx_events_name ON events(event_name);
CREATE INDEX idx_events_ts ON events(ts);
CREATE INDEX idx_events_name_ts ON events(event_name, ts);

-- ============================================
-- 4. Recreate page_views table with TEXT id
-- ============================================
CREATE TABLE page_views_new (
    id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL,
    path TEXT NOT NULL,
    referrer TEXT,
    ts_start INTEGER NOT NULL,
    ts_end INTEGER,
    duration_seconds INTEGER,
    max_scroll_percent INTEGER DEFAULT 0,
    FOREIGN KEY (session_id) REFERENCES sessions(id)
);

-- Migrate data with new UUIDs
INSERT INTO page_views_new (id, session_id, path, referrer, ts_start, ts_end, duration_seconds, max_scroll_percent)
SELECT
    lower(hex(randomblob(4)) || '-' || hex(randomblob(2)) || '-4' || substr(hex(randomblob(2)),2) || '-' || substr('89ab',abs(random()) % 4 + 1, 1) || substr(hex(randomblob(2)),2) || '-' || hex(randomblob(6))),
    session_id,
    path,
    referrer,
    ts_start,
    ts_end,
    duration_seconds,
    max_scroll_percent
FROM page_views;

DROP TABLE page_views;
ALTER TABLE page_views_new RENAME TO page_views;

CREATE INDEX idx_page_views_session ON page_views(session_id);
CREATE INDEX idx_page_views_path ON page_views(path);
CREATE INDEX idx_page_views_start ON page_views(ts_start);

-- Re-enable foreign keys
PRAGMA foreign_keys = ON;
