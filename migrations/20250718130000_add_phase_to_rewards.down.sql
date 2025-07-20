-- Remove phase column from rewards table
DROP INDEX IF EXISTS idx_rewards_user_phase;
DROP INDEX IF EXISTS idx_rewards_phase;
ALTER TABLE rewards DROP COLUMN phase;
