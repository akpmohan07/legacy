ALTER TABLE local_identity DROP COLUMN last_seen_at;
ALTER TABLE local_identity RENAME COLUMN first_seen_at TO first_seen;

DROP TABLE change_events;
DROP TABLE scans;

CREATE TABLE tools_old (
    id INTEGER PRIMARY KEY,
    name VARCHAR NOT NULL,
    source_id INTEGER NOT NULL REFERENCES sources(id),
    attributes TEXT,
    identifier VARCHAR NOT NULL
);
INSERT INTO tools_old (id, name, source_id, attributes, identifier)
    SELECT id, name, source_id, attributes, identifier FROM tools;
DROP TABLE tools;
ALTER TABLE tools_old RENAME TO tools;
CREATE UNIQUE INDEX tools_source_identifier_idx ON tools (source_id, identifier);

ALTER TABLE sources DROP COLUMN baselined_at;
ALTER TABLE sources DROP COLUMN installed_at;
ALTER TABLE sources DROP COLUMN first_seen_at;
ALTER TABLE sources DROP COLUMN version;
ALTER TABLE sources DROP COLUMN status;
