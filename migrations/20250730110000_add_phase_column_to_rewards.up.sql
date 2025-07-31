-- Add phase column to rewards table if it doesn't exist
ALTER TABLE rewards ADD COLUMN IF NOT EXISTS phase VARCHAR(20) DEFAULT 'phase1' NOT NULL;

-- Create index for phase if it doesn't exist
CREATE INDEX IF NOT EXISTS idx_rewards_phase ON rewards(phase);
CREATE INDEX IF NOT EXISTS idx_rewards_user_phase ON rewards(user_id, phase);
