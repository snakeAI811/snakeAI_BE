-- Consolidated schema for Snake AI Backend
-- This creates all tables and indexes in a single migration

-- Users table with all fields
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    twitter_id VARCHAR(255) NOT NULL UNIQUE,
    twitter_username VARCHAR(255),
    wallet_address VARCHAR(255),
    latest_claim_timestamp TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    selected_role VARCHAR(50) DEFAULT 'none',
    patron_qualification_score INTEGER DEFAULT 0,
    wallet_age_days INTEGER DEFAULT 0,
    community_score INTEGER DEFAULT 0,
    role VARCHAR(50) DEFAULT 'none',
    patron_status VARCHAR(50) DEFAULT 'none',
    locked_amount BIGINT DEFAULT 0,
    lock_start_timestamp TIMESTAMPTZ,
    lock_end_timestamp TIMESTAMPTZ,
    lock_duration_months INTEGER DEFAULT 0,
    last_yield_claim_timestamp TIMESTAMPTZ,
    total_yield_claimed BIGINT DEFAULT 0,
    user_claim_pda VARCHAR(255),
    initialized BOOLEAN DEFAULT false,
    vesting_pda VARCHAR(255),
    has_vesting BOOLEAN DEFAULT false,
    vesting_amount BIGINT DEFAULT 0,
    vesting_role_type VARCHAR(50),
    otc_swap_count INTEGER DEFAULT 0,
    total_burned BIGINT DEFAULT 0,
    dao_eligibility_revoked_at TIMESTAMPTZ
);

-- Sessions table
CREATE TABLE IF NOT EXISTS sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    session_id UUID NOT NULL DEFAULT gen_random_uuid() UNIQUE,
    user_agent TEXT NOT NULL,
    ip_address VARCHAR(55) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at TIMESTAMPTZ NOT NULL
);

-- Tweets table with all fields
CREATE TABLE IF NOT EXISTS tweets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    tweet_id VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    mining_phase SMALLINT NOT NULL DEFAULT 1,
    rewarded BOOLEAN DEFAULT FALSE,
    reward_amount BIGINT DEFAULT 0
);

-- Rewards table with all fields
CREATE TABLE IF NOT EXISTS rewards (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    tweet_id UUID NOT NULL REFERENCES tweets(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    available BOOLEAN NOT NULL DEFAULT TRUE,
    message_sent BOOLEAN NOT NULL DEFAULT FALSE,
    transaction_signature VARCHAR(255),
    reward_amount BIGINT NOT NULL DEFAULT 0,
    wallet_address VARCHAR(255),
    block_time TIMESTAMPTZ,
    media_id VARCHAR(255),
    media_id_expires_at TIMESTAMPTZ,
    phase VARCHAR(20) DEFAULT 'phase1' NOT NULL
);

-- Values table for key-value storage
CREATE TABLE IF NOT EXISTS values (
    key VARCHAR(255) PRIMARY KEY NOT NULL,
    value VARCHAR(255) NOT NULL
);

-- All indexes
CREATE INDEX idx_users_twitter_id ON users(twitter_id);
CREATE INDEX idx_users_wallet_address ON users(wallet_address);
CREATE INDEX idx_users_selected_role ON users(selected_role);
CREATE INDEX idx_users_patron_qualification_score ON users(patron_qualification_score);
CREATE INDEX idx_users_role ON users(role);
CREATE INDEX idx_users_patron_status ON users(patron_status);
CREATE INDEX idx_users_user_claim_pda ON users(user_claim_pda);
CREATE INDEX idx_users_vesting_pda ON users(vesting_pda);

CREATE INDEX idx_sessions_user_id ON sessions(user_id);
CREATE INDEX idx_sessions_session_id ON sessions(session_id);

CREATE INDEX idx_tweets_user_id ON tweets(user_id);
CREATE INDEX idx_tweets_mining_phase ON tweets(mining_phase);
CREATE INDEX idx_tweets_user_phase ON tweets(user_id, mining_phase);
CREATE INDEX idx_tweets_rewarded ON tweets(rewarded);
CREATE INDEX idx_tweets_user_rewarded ON tweets(user_id, rewarded);

CREATE INDEX idx_rewards_user_id ON rewards(user_id);
CREATE INDEX idx_rewards_tweet_id ON rewards(tweet_id);
CREATE INDEX idx_rewards_phase ON rewards(phase);
CREATE INDEX idx_rewards_user_phase ON rewards(user_id, phase);
