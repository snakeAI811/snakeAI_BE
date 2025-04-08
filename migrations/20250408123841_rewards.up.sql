-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS rewards (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
        reward_amount BIGINT DEFAULT 0,
        tx_id VARCHAR(255),
        created_at TIMESTAMPTZ DEFAULT now(),
        timestamp TIMESTAMPTZ
    );
