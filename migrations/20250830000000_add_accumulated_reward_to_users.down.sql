-- Remove accumulated_reward field from users table
DROP INDEX IF EXISTS idx_users_accumulated_reward;
ALTER TABLE users DROP COLUMN IF EXISTS accumulated_reward;