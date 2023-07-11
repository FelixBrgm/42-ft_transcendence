use crate::models::{NewClient, Client};
use crate::db::get_connection;
use diesel::prelude::*;

pub fn create_client(title: &str) {
	println!("creating client... {:?}", title);
	use crate::schema::clients::dsl::*;

	let new_client = NewClient {
		title: "test",
		is_online: true,
	};

	let connection = get_connection();
	match connection {
		Ok(con) => {
			diesel::insert_into(clients)
				.values(&new_client)
				.execute(&con)
				.expect("Error saving new Client");
		},
		Err(_) => {println!("failed to get connection")},
	};
}