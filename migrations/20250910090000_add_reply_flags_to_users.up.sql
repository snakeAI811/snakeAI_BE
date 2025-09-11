-- Add flags to track whether a user has received success/failed replies
ALTER TABLE users
    ADD COLUMN IF NOT EXISTS success_msg_flag BOOLEAN NOT NULL DEFAULT FALSE,
    ADD COLUMN IF NOT EXISTS failed_msg_flag BOOLEAN NOT NULL DEFAULT FALSE;