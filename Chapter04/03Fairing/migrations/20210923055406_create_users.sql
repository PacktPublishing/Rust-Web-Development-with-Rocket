-- Add migration script here
CREATE TABLE IF NOT EXISTS users
(
    uuid   UUID PRIMARY KEY,
    name   VARCHAR NOT NULL,
    age    SMALLINT NOT NULL DEFAULT 0,
    grade  SMALLINT NOT NULL DEFAULT 0,
    active BOOL NOT NULL DEFAULT TRUE
);
CREATE INDEX name_active_idx ON users(name, active);
