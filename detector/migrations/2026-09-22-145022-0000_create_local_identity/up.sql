CREATE TABLE local_identity (
    id INTEGER PRIMARY KEY,
    platform_uuid VARCHAR NOT NULL UNIQUE,
    device_name VARCHAR,
    username VARCHAR,
    attributes TEXT,
    first_seen TEXT NOT NULL DEFAULT (datetime('now'))
);
