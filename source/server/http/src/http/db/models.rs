use super::schema::*;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::Serialize;

// ----------- Users  -----------------
#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = app_user)]
pub struct NewUser {
    pub id: i32,
    pub login: String,
    pub avatar: String,
}

#[derive(Queryable, PartialEq, AsChangeset, Debug, Clone, Default)]
#[diesel(table_name = app_user)]
pub struct UpdateUser {
    pub id: i32,
    pub login: Option<String>,
    pub avatar: Option<String>,
    pub password: Option<Vec<u8>>,
    pub status: Option<String>,
    pub wins: Option<i32>,
    pub losses: Option<i32>,
}

#[derive(Debug, Queryable, AsChangeset, Serialize)]
#[diesel(table_name = app_user)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub avatar: String,
    pub password: Option<Vec<u8>>,
    pub status: String,
    pub wins: i32,
    pub losses: i32,
}

// ----------- Rooms -----------------

#[derive(Insertable, Debug)]
#[diesel(table_name = chat_rooms)]
pub struct NewChatRoom {
    pub owner: i32,
    pub name: String,
    pub topic: Option<String>,
    pub is_public: bool,
    pub password: Option<Vec<u8>>,
}

#[derive(AsChangeset, Debug)]
#[diesel(table_name = chat_rooms)]
pub struct UpdateChatRoom {
    pub id: i32,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub is_public: Option<bool>,
    pub password: Option<Option<Vec<u8>>>,
}

#[derive(Queryable, Debug)]
#[diesel(table_name = chat_rooms)]
pub struct ChatRoom {
    pub id: i32,
    pub owner: i32,
    pub name: String,
    pub topic: Option<String>,
    pub is_public: bool,
    pub password: Option<Vec<u8>>,
}

// ----------- Messages --------------

#[derive(Insertable, Debug, Queryable)]
#[diesel(table_name = chat_messages)]
pub struct NewMessage {
    pub sender_id: i32,
    pub room_id: i32,
    pub message: String,
    pub timestamp: NaiveDateTime,
}

// ----------- Connections  ----------

#[derive(Insertable, Debug, Queryable)]
#[diesel(table_name = room_user_connection)]
pub struct RoomUserConnection {
    pub user_id: i32,
    pub room_id: i32,
}

#[derive(Insertable, Debug, Queryable)]
#[diesel(table_name = user_room_connection)]
pub struct UserRoomConnection {
    pub user_id: i32,
    pub room_id: i32,
}
