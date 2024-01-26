// @generated automatically by Diesel CLI.

diesel::table! {
    app_user (id) {
        id -> Int4,
        #[max_length = 255]
        intra -> Varchar,
        #[max_length = 255]
        alias -> Varchar,
        #[max_length = 255]
        avatar -> Varchar,
        password -> Nullable<Bytea>,
        #[max_length = 255]
        status -> Varchar,
        wins -> Int4,
        losses -> Int4,
    }
}

diesel::table! {
    blocked_users (id) {
        id -> Int4,
        user_id -> Int4,
        blocked_user_id -> Int4,
    }
}

diesel::table! {
    chat_messages (id) {
        id -> Int4,
        room_id -> Int4,
        sender_id -> Int4,
        message -> Text,
        timestamp -> Timestamp,
    }
}

diesel::table! {
    chat_rooms (id) {
        id -> Int4,
        user1 -> Int4,
        user2 -> Int4,
    }
}

diesel::table! {
    friend_ship (id) {
        id -> Int4,
        user1 -> Int4,
        user2 -> Int4,
    }
}

diesel::table! {
    game_match (id) {
        id -> Int4,
        winner -> Int4,
        looser -> Int4,
        timestamp -> Timestamp,
    }
}

diesel::joinable!(chat_messages -> app_user (sender_id));
diesel::joinable!(chat_messages -> chat_rooms (room_id));

diesel::allow_tables_to_appear_in_same_query!(
    app_user,
    blocked_users,
    chat_messages,
    chat_rooms,
    friend_ship,
    game_match,
);
