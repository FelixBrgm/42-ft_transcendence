use super::schema::*;
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
