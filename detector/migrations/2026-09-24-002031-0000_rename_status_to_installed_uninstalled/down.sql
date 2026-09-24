-- Restores the old present/removed vocabulary. Every row comes back as 'present':
-- fine while nothing writes status yet, but it would lose 'uninstalled' rows once something does.
ALTER TABLE sources DROP COLUMN status;
ALTER TABLE sources ADD COLUMN status VARCHAR CHECK (status IN ('present', 'removed'));

ALTER TABLE tools DROP COLUMN status;
ALTER TABLE tools ADD COLUMN status VARCHAR NOT NULL DEFAULT 'present' CHECK (status IN ('present', 'removed'));
