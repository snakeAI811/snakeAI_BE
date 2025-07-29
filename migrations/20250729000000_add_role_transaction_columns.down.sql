-- Remove role transaction tracking columns
ALTER TABLE users 
DROP COLUMN IF EXISTS role_transaction_signature,
DROP COLUMN IF EXISTS role_updated_at;

-- Drop index
DROP INDEX IF EXISTS idx_users_role_transaction_signature;
