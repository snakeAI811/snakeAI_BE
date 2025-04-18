-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS users (
        id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
        twitter_id VARCHAR(255) NOT NULL UNIQUE,
        twitter_username VARCHAR(255),
        wallet_address VARCHAR(255),
        latest_claim_timestamp TIMESTAMPTZ,
        created_at TIMESTAMPTZ NOT NULL DEFAULT now()
    );

-- Indexes for faster lookups
CREATE INDEX idx_users_twitter_id ON users(twitter_id);
CREATE INDEX idx_users_wallet_address ON users(wallet_address);
