-- Add up migration script here

ALTER TABLE rewards
ADD COLUMN media_id VARCHAR(255),
ADD COLUMN media_id_expires_at TIMESTAMPTZ;
