// @generated automatically by Diesel CLI.

diesel::table! {
    app_user (id) {
        id -> Int4,
        #[max_length = 255]
        login -> Varchar,
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
        owner -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        topic -> Nullable<Varchar>,
        is_public -> Bool,
        password -> Nullable<Bytea>,
    }
}

diesel::table! {
    user_chat_room (id) {
        id -> Int4,
        user_id -> Int4,
        room_id -> Int4,
    }
}

diesel::joinable!(chat_messages -> app_user (sender_id));
diesel::joinable!(chat_messages -> chat_rooms (room_id));
diesel::joinable!(chat_rooms -> app_user (owner));
diesel::joinable!(user_chat_room -> app_user (user_id));
diesel::joinable!(user_chat_room -> chat_rooms (room_id));

diesel::allow_tables_to_appear_in_same_query!(
    app_user,
    chat_messages,
    chat_rooms,
    user_chat_room,
);
