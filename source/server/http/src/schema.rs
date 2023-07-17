// @generated automatically by Diesel CLI.

diesel::table! {
    clients (id) {
        id -> Int4,
        title -> Text,
        is_online -> Bool,
    }
}
