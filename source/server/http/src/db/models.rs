use crate::db::schema::*;
use diesel::{Queryable, Insertable, AsChangeset};
use serde::Serialize;

// ----------- test CLIENT STRUCT -----------------
#[derive (Insertable)]
#[diesel(table_name = clients)]
pub struct NewClient<'a> {
	pub title: &'a str,
	pub is_online: bool,
}

#[derive(Debug, Queryable, AsChangeset)]
#[diesel(table_name = clients)]
pub struct Client {
	pub id: i32,
	pub title: String,
	pub is_online: bool,
}

// ----------- Users  -----------------
#[derive (Insertable, Debug, Clone)]
#[diesel(table_name = app_user)]
pub struct NewUser {
	pub id: i32,
	pub login: String,
	pub avatar: String,
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