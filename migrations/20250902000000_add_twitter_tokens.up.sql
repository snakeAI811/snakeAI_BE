-- Add Twitter OAuth tokens to users table
ALTER TABLE users 
ADD COLUMN twitter_access_token TEXT,
ADD COLUMN twitter_refresh_token TEXT,
ADD COLUMN twitter_token_expires_at TIMESTAMPTZ;

-- Create index for efficient token lookups
CREATE INDEX idx_users_twitter_access_token ON users(twitter_access_token) WHERE twitter_access_token IS NOT NULL;