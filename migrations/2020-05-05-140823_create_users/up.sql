-- Your SQL goes here

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  discord_id VARCHAR NOT NULL,
  username TEXT NOT NULL
)
