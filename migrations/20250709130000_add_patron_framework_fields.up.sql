-- Add patron framework fields to users table
ALTER TABLE users
ADD COLUMN role VARCHAR(50) DEFAULT 'none',
ADD COLUMN patron_status VARCHAR(50) DEFAULT 'none',
ADD COLUMN locked_amount BIGINT DEFAULT 0,
ADD COLUMN lock_start_timestamp TIMESTAMPTZ,
ADD COLUMN lock_end_timestamp TIMESTAMPTZ,
ADD COLUMN lock_duration_months INTEGER DEFAULT 0,
ADD COLUMN last_yield_claim_timestamp TIMESTAMPTZ,
ADD COLUMN total_yield_claimed BIGINT DEFAULT 0,
ADD COLUMN user_claim_pda VARCHAR(255),
ADD COLUMN initialized BOOLEAN DEFAULT false,
ADD COLUMN vesting_pda VARCHAR(255),
ADD COLUMN has_vesting BOOLEAN DEFAULT false,
ADD COLUMN vesting_amount BIGINT DEFAULT 0,
ADD COLUMN vesting_role_type VARCHAR(50),
ADD COLUMN otc_swap_count INTEGER DEFAULT 0,
ADD COLUMN total_burned BIGINT DEFAULT 0,
ADD COLUMN dao_eligibility_revoked_at TIMESTAMPTZ;

-- Add indexes for performance
CREATE INDEX idx_users_role ON users(role);
CREATE INDEX idx_users_patron_status ON users(patron_status);
CREATE INDEX idx_users_user_claim_pda ON users(user_claim_pda);
CREATE INDEX idx_users_vesting_pda ON users(vesting_pda);
