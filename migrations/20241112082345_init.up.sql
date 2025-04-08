-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS users (
        id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
        first_name VARCHAR(255) NOT NULL,
        middle_name VARCHAR(255) NOT NULL,
        second_name VARCHAR(255) NOT NULL,
        user_name VARCHAR(255) NOT NULL,
        email VARCHAR(255),
        password VARCHAR(255),
        email_created_at TIMESTAMP,
        email_updated_at TIMESTAMP,
        gmail VARCHAR(255),
        gmail_created_at TIMESTAMP,
        gmail_updated_at TIMESTAMP,
        gender SMALLINT DEFAULT 0,
        country VARCHAR(255),
        city VARCHAR(255),
        created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
    );
