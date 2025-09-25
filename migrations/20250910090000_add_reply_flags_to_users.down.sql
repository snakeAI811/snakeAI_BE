-- Revert reply flags
ALTER TABLE users
    DROP COLUMN IF EXISTS success_msg_flag,
    DROP COLUMN IF EXISTS failed_msg_flag;