-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS rewards (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
        tweet_id UUID NOT NULL REFERENCES tweets(id) ON DELETE CASCADE,
        created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
        available BOOLEAN NOT NULL DEFAULT TRUE,
        message_sent BOOLEAN NOT NULL DEFAULT FALSE,
        -- solana tranaction
        transaction_signature VARCHAR(255),
        reward_amount BIGINT NOT NULL DEFAULT 0,
        wallet_address VARCHAR(255),
        block_time TIMESTAMPTZ
    );

-- Indexes for faster lookups
CREATE INDEX idx_rewards_user_id ON rewards(user_id);
CREATE INDEX idx_rewards_tweet_id ON rewards(tweet_id);
