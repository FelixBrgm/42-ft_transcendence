-- Your SQL goes here
CREATE TABLE user_room_connection (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    room_id INT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES app_user (id),
    FOREIGN KEY (room_id) REFERENCES chat_rooms (id),
    UNIQUE (user_id, room_id)
);