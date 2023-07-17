use crate::schema::clients;

// ----------- test CLIENT STRUCT -----------------
#[derive (Insertable)]
#[table_name = "clients"]
pub struct NewClient<'a> {
	pub title: &'a str,
	pub is_online: bool,
}

#[derive(Debug, Queryable, AsChangeset)]
pub struct Client {
	pub id: i32,
	pub title: String,
	pub is_online: bool,
}

// ----------- other  -----------------

