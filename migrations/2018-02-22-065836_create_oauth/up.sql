-- Your SQL goes here
CREATE TABLE oauth_tokens (
    id INTEGER PRIMARY KEY NOT NULL,
    oauth_token VARCHAR NOT NULL,
    team_id VARCHAR NOT NULL
)
