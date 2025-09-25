-- Remove is_following column from users table
ALTER TABLE users
DROP COLUMN IF EXISTS is_following;
