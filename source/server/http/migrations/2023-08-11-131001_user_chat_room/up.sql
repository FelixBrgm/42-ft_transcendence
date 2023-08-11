-- Your SQL goes here
-- The UNIQUE constraint ensures that a user can only be a member of a chat room once.
CREATE TABLE user_chat_room (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    room_id INT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES app_user (id),
    FOREIGN KEY (room_id) REFERENCES chat_rooms (id),
    UNIQUE (user_id, room_id)
);