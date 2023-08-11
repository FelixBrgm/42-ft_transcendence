-- Your SQL goes here
CREATE TABLE chat_messages (
    id SERIAL PRIMARY KEY,
    room_id INT NOT NULL,
    sender_id INT NOT NULL,
    message TEXT NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    FOREIGN KEY (room_id) REFERENCES chat_rooms (id),
    FOREIGN KEY (sender_id) REFERENCES app_user (id)
);