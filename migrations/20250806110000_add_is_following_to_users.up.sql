-- Add is_following column to users table
ALTER TABLE users
ADD COLUMN IF NOT EXISTS is_following BOOLEAN NOT NULL DEFAULT FALSE;
