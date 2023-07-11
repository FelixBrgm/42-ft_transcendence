use crate::models::{NewClient, Client};
use crate::db::get_connection;
use diesel::prelude::*;

pub fn insert_client(name: &str) {
	println!("creating client... {:?}", name);
	use crate::schema::clients::dsl::*;
	let new_client = NewClient {
		title: name,
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

pub fn find_client(name: &str) -> Option<Client>
{
	println!("searching client... {:?}", name);
	None
}

// pub fn remove_user()

pub fn show_clients() {
	println!("showing clients...");
	use crate::schema::clients::dsl::*;

	let connection = get_connection();
	match connection {
		Ok(con) => {
			let results = clients
			.load::<Client>(&con)
			.expect("error loading clients");

			for client in results{
				println!("{:?}", client);
			}
		},
		Err(_) => {println!("failed to get connection")},
	};
}