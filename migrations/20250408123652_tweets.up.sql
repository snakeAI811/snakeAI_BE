-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS tweets (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
        tweet_id VARCHAR(255) NOT NULL UNIQUE,
        created_at TIMESTAMPTZ NOT NULL DEFAULT now()
    );

-- Indexes for faster lookups
CREATE INDEX idx_tweets_user_id ON tweets(user_id);
