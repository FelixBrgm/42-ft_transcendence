use super::schema::*;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

// ----------- Users  -----------------

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = app_user)]
pub struct NewUser {
    pub id: i32,
    pub intra: String,
    pub alias: String,
    pub avatar: String,
}

#[derive(Queryable, PartialEq, AsChangeset, Debug, Clone, Default, Deserialize)]
#[diesel(table_name = app_user)]
pub struct UpdateUser {
    pub alias: Option<String>,
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
    pub intra: String,
    pub alias: String,
    pub avatar: String,
    pub password: Option<Vec<u8>>,
    pub status: String,
    pub wins: i32,
    pub losses: i32,
}

// --------- Friends -------------------

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = friend_ship)]
pub struct Friendship {
    pub user1: i32,
    pub user2: i32,
}

// --------- Blocked -------------------

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = blocked_users)]
pub struct Blocked {
    pub id: i32,
    pub blocked_user_id: i32,
}

// --------- Rooms ---------------------

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = chat_rooms)]
pub struct NewChatRoom {
    pub user1: i32,
    pub user2: i32,
}

#[derive(Insertable, Debug, Deserialize, Serialize, Queryable)]
#[diesel(table_name = chat_rooms)]
pub struct ChatRoom {
	pub id: i32,
    pub user1: i32,
    pub user2: i32,
}

// // ----------- Messages --------------

#[derive(Insertable, Debug, Queryable, Deserialize)]
#[diesel(table_name = chat_messages)]
pub struct NewMessage {
    pub sender_id: i32,
    pub room_id: i32,
    pub message: String,
}

#[derive(Insertable, Debug, Queryable, Serialize)]
#[diesel(table_name = chat_messages)]
pub struct Message {
    pub id: i32,
    pub room_id: i32,
    pub sender_id: i32,
    pub message: String,
    pub timestamp: NaiveDateTime,
}
