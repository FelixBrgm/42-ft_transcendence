-- Your SQL goes here
	CREATE TABLE chat_rooms (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    topic VARCHAR(255),
    is_public BOOLEAN NOT NULL,
    password BYTEA NULL
);