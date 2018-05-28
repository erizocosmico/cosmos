CREATE TABLE users (
    id UUID PRIMARY KEY,
    username VARCHAR(60) NOT NULL,
    password VARCHAR(100) NOT NULL,
    email VARCHAR(100) NOT NULL,
    active BOOLEAN NOT NULL DEFAULT 'f'
);

CREATE TABLE user_sessions (
    id UUID PRIMARY KEY,
    token UUID NOT NULL,
    user_id UUID NOT NULL REFERENCES users(id),
    expires_at TIMESTAMPTZ NOT NULL
);