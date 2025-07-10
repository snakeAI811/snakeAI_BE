-- Add phase column to tweets table to track mining phases
ALTER TABLE tweets ADD COLUMN mining_phase SMALLINT NOT NULL DEFAULT 1;

-- Add index for phase-based queries
CREATE INDEX idx_tweets_mining_phase ON tweets(mining_phase);
CREATE INDEX idx_tweets_user_phase ON tweets(user_id, mining_phase);
