-- Remove phase tracking from tweets table
DROP INDEX IF EXISTS idx_tweets_user_phase;
DROP INDEX IF EXISTS idx_tweets_mining_phase;
ALTER TABLE tweets DROP COLUMN mining_phase;
