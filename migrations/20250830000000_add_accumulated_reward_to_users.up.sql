-- Add accumulated_reward field to users table
ALTER TABLE users ADD COLUMN accumulated_reward BIGINT DEFAULT 0;

-- Create index for accumulated_reward for better query performance
CREATE INDEX idx_users_accumulated_reward ON users(accumulated_reward);