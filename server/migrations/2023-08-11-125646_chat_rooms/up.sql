-- Your SQL goes here
	CREATE TABLE chat_rooms (
    id SERIAL PRIMARY KEY,
    user1 INT NOT NULL,
    user2 INT NOT NULL,
    FOREIGN KEY (user1) REFERENCES app_user (id),
    FOREIGN KEY (user2) REFERENCES  app_user (id),
    UNIQUE (user1, user2)
);
