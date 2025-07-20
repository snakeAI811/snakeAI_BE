-- Add phase column to rewards table for Phase 1/Phase 2 tracking
ALTER TABLE rewards ADD COLUMN phase VARCHAR(20) DEFAULT 'phase1' NOT NULL;

-- Create index for efficient phase-based queries
CREATE INDEX IF NOT EXISTS idx_rewards_phase ON rewards(phase);
CREATE INDEX IF NOT EXISTS idx_rewards_user_phase ON rewards(user_id, phase);
