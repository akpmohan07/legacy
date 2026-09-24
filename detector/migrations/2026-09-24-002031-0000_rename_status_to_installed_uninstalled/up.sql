-- Status uses the same words as the events: installed / uninstalled.
-- Drop and re-add (no table rebuild): a column-level CHECK can only be changed that way in SQLite.
-- Existing tools all become 'installed' via the column default; sources are still NULL.
ALTER TABLE tools DROP COLUMN status;
ALTER TABLE tools ADD COLUMN status VARCHAR NOT NULL DEFAULT 'installed' CHECK (status IN ('installed', 'uninstalled'));

ALTER TABLE sources DROP COLUMN status;
ALTER TABLE sources ADD COLUMN status VARCHAR CHECK (status IN ('installed', 'uninstalled'));
