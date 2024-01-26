-- Your SQL goes here
CREATE TABLE blocked_users (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    blocked_user_id INT NOT NULL,
    
    FOREIGN KEY (user_id) REFERENCES app_user(id),
    FOREIGN KEY (blocked_user_id) REFERENCES app_user(id)
);
