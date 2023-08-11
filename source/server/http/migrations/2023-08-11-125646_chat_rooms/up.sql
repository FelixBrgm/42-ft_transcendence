-- Your SQL goes here
	CREATE TABLE chat_rooms (
    id SERIAL PRIMARY KEY,
	owner INT NOT NULL,
    name VARCHAR(255) NOT NULL,
    topic VARCHAR(255),
    is_public BOOLEAN NOT NULL,
    password BYTEA NULL,
    FOREIGN KEY (owner) REFERENCES app_user (id)
);