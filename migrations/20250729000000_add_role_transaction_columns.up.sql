-- Add columns for role selection transaction tracking
ALTER TABLE users 
ADD COLUMN IF NOT EXISTS role_transaction_signature VARCHAR(255),
ADD COLUMN IF NOT EXISTS role_updated_at TIMESTAMPTZ;

-- Create index for the transaction signature
CREATE INDEX IF NOT EXISTS idx_users_role_transaction_signature ON users(role_transaction_signature);
