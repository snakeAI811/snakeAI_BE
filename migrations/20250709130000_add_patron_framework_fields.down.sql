-- Remove patron framework fields from users table
ALTER TABLE users
DROP COLUMN role,
DROP COLUMN patron_status,
DROP COLUMN locked_amount,
DROP COLUMN lock_start_timestamp,
DROP COLUMN lock_end_timestamp,
DROP COLUMN lock_duration_months,
DROP COLUMN last_yield_claim_timestamp,
DROP COLUMN total_yield_claimed,
DROP COLUMN user_claim_pda,
DROP COLUMN initialized,
DROP COLUMN vesting_pda,
DROP COLUMN has_vesting,
DROP COLUMN vesting_amount,
DROP COLUMN vesting_role_type,
DROP COLUMN otc_swap_count,
DROP COLUMN total_burned,
DROP COLUMN dao_eligibility_revoked_at;

-- Drop indexes
DROP INDEX IF EXISTS idx_users_role;
DROP INDEX IF EXISTS idx_users_patron_status;
DROP INDEX IF EXISTS idx_users_user_claim_pda;
DROP INDEX IF EXISTS idx_users_vesting_pda;
