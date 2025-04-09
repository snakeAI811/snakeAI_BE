-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS sessions (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
        session_id UUID NOT NULL DEFAULT gen_random_uuid() UNIQUE,
        user_agent TEXT NOT NULL,
        ip_address VARCHAR(55) NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
        expires_at TIMESTAMPTZ NOT NULL
    );
