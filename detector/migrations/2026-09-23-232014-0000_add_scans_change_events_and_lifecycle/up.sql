-- The app's bundled SQLite enforces foreign keys, so a parent table (sources) is never dropped here.

-- sources: all new columns are nullable, so they are added in place.
-- Seeded rows are a catalog: status / first_seen_at stay NULL until discovery actually sees the source.
ALTER TABLE sources ADD COLUMN status VARCHAR CHECK (status IN ('present', 'removed'));
ALTER TABLE sources ADD COLUMN version VARCHAR;
ALTER TABLE sources ADD COLUMN first_seen_at TEXT;
ALTER TABLE sources ADD COLUMN installed_at TEXT;
ALTER TABLE sources ADD COLUMN baselined_at TEXT;

-- tools: rebuilt (a child table, safe to drop) because SQLite cannot ALTER in a non-constant default.
CREATE TABLE tools_new (
    id INTEGER PRIMARY KEY,
    name VARCHAR NOT NULL,
    source_id INTEGER NOT NULL REFERENCES sources(id),
    attributes TEXT,
    identifier VARCHAR NOT NULL,
    status VARCHAR NOT NULL DEFAULT 'present' CHECK (status IN ('present', 'removed')),
    first_seen_at TEXT NOT NULL DEFAULT (datetime('now')),
    installed_at TEXT
);
INSERT INTO tools_new (id, name, source_id, attributes, identifier)
    SELECT id, name, source_id, attributes, identifier FROM tools;
DROP TABLE tools;
ALTER TABLE tools_new RENAME TO tools;
CREATE UNIQUE INDEX tools_source_identifier_idx ON tools (source_id, identifier);

-- one row per scan of one source. finished_at is set only when the scan succeeded
-- (a source confirmed absent counts as success), so it doubles as the source's freshness.
CREATE TABLE scans (
    id INTEGER PRIMARY KEY,
    run_id TEXT NOT NULL,
    source_id INTEGER NOT NULL REFERENCES sources(id),
    triggered_by TEXT NOT NULL DEFAULT 'manual',
    started_at TEXT NOT NULL DEFAULT (datetime('now')),
    finished_at TEXT
);
CREATE INDEX scans_source_idx ON scans (source_id, id);
CREATE INDEX scans_run_idx ON scans (run_id);

-- append-only history; a source event has tool_id NULL
CREATE TABLE change_events (
    id INTEGER PRIMARY KEY,
    scan_id INTEGER NOT NULL REFERENCES scans(id),
    tool_id INTEGER REFERENCES tools(id),
    event_type VARCHAR NOT NULL CHECK (event_type IN ('installed', 'updated', 'uninstalled')),
    changes TEXT NOT NULL CHECK (json_valid(changes)),
    occurred_at TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX change_events_tool_idx ON change_events (tool_id, id);

-- local_identity: heartbeat for the whole installation, updated once per run
ALTER TABLE local_identity RENAME COLUMN first_seen TO first_seen_at;
ALTER TABLE local_identity ADD COLUMN last_seen_at TEXT;
UPDATE local_identity SET last_seen_at = datetime('now');
