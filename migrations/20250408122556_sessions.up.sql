-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS sessions (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
        token TEXT NOT NULL UNIQUE,
        user_agent TEXT,
        ip_address INET,
        created_at TIMESTAMPTZ DEFAULT now(),
        expired_at TIMESTAMPTZ
    );
