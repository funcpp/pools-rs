-- Your SQL goes here
CREATE TABLE channels (
    id SERIAL PRIMARY KEY,
    name VARCHAR(511) NOT NULL,
    parent_id INT REFERENCES channels(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);