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
    clients (id) {
        id -> Int4,
        title -> Text,
        is_online -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    app_user,
    clients,
);
