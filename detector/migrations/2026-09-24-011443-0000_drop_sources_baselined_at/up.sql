-- The baseline is derived instead of stored: sources.first_seen_at is set only when the source's
-- first successful scan is recorded, so "first_seen_at is empty" means the next scan is the baseline.
ALTER TABLE sources DROP COLUMN baselined_at;
