-- Your SQL goes here
CREATE TABLE game_match (
    id SERIAL PRIMARY KEY,
    winner INT NOT NULL,
    looser INT NOT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (winner) REFERENCES app_user (id),
    FOREIGN KEY (looser) REFERENCES app_user (id)
);
