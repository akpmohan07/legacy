CREATE TABLE sources (
    id INTEGER PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    type VARCHAR NOT NULL CHECK (type IN ('package_manager', 'native_apps', 'extension'))
);

CREATE TABLE tools (
    id INTEGER PRIMARY KEY,
    name VARCHAR NOT NULL,
    source_id INTEGER NOT NULL REFERENCES sources(id),
    attributes TEXT,
    identifier VARCHAR NOT NULL
);

CREATE UNIQUE INDEX tools_source_identifier_idx ON tools (source_id, identifier);
