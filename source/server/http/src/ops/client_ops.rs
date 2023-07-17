use crate::models::{NewClient, Client};
use crate::schema::clients::dsl::*;
use crate::db::get_connection;
use diesel::prelude::*;

// all of these just get a new connection rn

pub fn insert(name: &str) {
	println!("inserting client... {:?}", name);
	let connection = get_connection();

	let new_client = NewClient {
		title: name,
		is_online: true,
	};
	
	diesel::insert_into(clients)
		.values(&new_client)
		.execute(&connection)
		.expect("Error saving new Client");
}

pub fn remove(name: &str) -> usize {
	println!("removing client... {:?}", name);
	let connection = get_connection();

	diesel::delete(clients.filter(title.eq(name)))
	.execute(&connection)
	.expect("Failed to delete CLient {name}")
}

pub fn exists(name: &str) -> bool {
	let connection = get_connection();
	
	clients
	.filter(title.eq(name))
	.first::<Client>(&connection)
	.is_ok()
}

pub fn search(name: &str) -> Option<Client {
	println!("searching for client... {:?}", name);
	let connection = get_connection();

	clients
	.filter(title.eq(name))
	.first::<Client>(&connection)
	.ok()
}

pub fn show() {
	println!("showing clients...");
	let connection = get_connection();

	let results = clients
	.load::<Client>(&connection)
	.unwrap_or(vec![]);

	for client in results{
		println!("{:?}", client);
	}
}