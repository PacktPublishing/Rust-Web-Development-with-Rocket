-- Add migration script here
CREATE TABLE IF NOT EXISTS posts
(
    uuid       UUID PRIMARY KEY,
    user_uuid  UUID NOT NULL,
    post_type  INTEGER NOT NULL DEFAULT 0,
    content    VARCHAR NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_uuid) REFERENCES "users" (uuid)
);
