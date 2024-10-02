-- Your SQL goes here
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    channel_id INT NOT NULL,
    author_id CHAR(24) NOT NULL, -- the main db is mongodb...
    title VARCHAR(1023) NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ,

    CONSTRAINT fk_channel_id
        FOREIGN KEY (channel_id)
        REFERENCES channels(id)
);

CREATE OR REPLACE FUNCTION set_current_timestamp() RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_current_timestamp
    BEFORE UPDATE ON posts
    FOR EACH ROW
    EXECUTE FUNCTION set_current_timestamp();

CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    post_id INT NOT NULL,
    parent_id INT,
    author_id CHAR(24) NOT NULL, -- the main db is mongodb...
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    is_deleted BOOLEAN DEFAULT FALSE,
    deleted_at TIMESTAMPTZ,

    CONSTRAINT fk_post_id
        FOREIGN KEY (post_id)
        REFERENCES posts(id),
    CONSTRAINT fk_parent_id
        FOREIGN KEY (parent_id)
        REFERENCES comments(id)
);