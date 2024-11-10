DO $$
    BEGIN
        IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'gender') THEN
            CREATE TYPE gender AS ENUM ('male', 'female', 'other');
        END IF;
    END $$;

DO $$
    BEGIN
        IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'user_status') THEN
            CREATE TYPE user_status AS ENUM ('online', 'offline', 'idle', 'dnd');
        END IF;
    END $$;

CREATE TABLE IF NOT EXISTS users
(
    id            UUID PRIMARY KEY,
    first_name    TEXT        NOT NULL,
    last_name     TEXT,
    nickname      TEXT        NOT NULL UNIQUE,
    email         TEXT        NOT NULL UNIQUE,
    avatar        TEXT        NOT NULL,
    password_hash TEXT        NOT NULL,
    gender        gender      NOT NULL,
    birthday      TIMESTAMPTZ,
    status        user_status NOT NULL,
    roles         TEXT[]      NOT NULL DEFAULT ARRAY []::TEXT[],
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);