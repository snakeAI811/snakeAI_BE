-- Add down migration script here

ALTER TABLE rewards
DROP COLUMN media_id,
DROP COLUMN media_id_expires_at;