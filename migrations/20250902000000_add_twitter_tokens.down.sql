-- Remove Twitter OAuth tokens from users table
DROP INDEX IF EXISTS idx_users_twitter_access_token;

ALTER TABLE users 
DROP COLUMN IF EXISTS twitter_access_token,
DROP COLUMN IF EXISTS twitter_refresh_token,
DROP COLUMN IF EXISTS twitter_token_expires_at;