CREATE TYPE gender AS ENUM ('male', 'female', 'other');
CREATE TYPE user_status AS ENUM ('online', 'offline', 'idle', 'dnd');

CREATE TABLE users
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