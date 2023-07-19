// #[macro_use]
// extern crate diesel;

// #![allow(dead_code)]
// #![allow(unused_imports)]
// #[allow(unused_variables)]

mod db;

use db::wrapper::Database;
use actix_web::{web, App, HttpServer};

// testing
use crate::db::models::NewClient;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

	let db = setup_database();

	db.show_clients();

	// let new_client = NewClient {
    //     title: "Peter",
    //     is_online: true,
    // };

	// match db.add_client(&new_client) {
    //     Ok(_) => println!("Client inserted successfully!"),
    //     Err(e) => eprintln!("Failed to insert client: {}", e),
    // }

	// match db.get_client_id(21) {
	// 	Ok(client) => println!("Client found {:?}", client),
    //     Err(e) => eprintln!("Client not found :("),
	// }
	
	// // let found = db.get_client_name(new_client.title);
	// // match found {
	// // 	Ok(mut client) => {
	// // 		println!("Client found {:?}", client);
	// // 		client.title = "emet".to_string();
	// // 		match db.set_client(&client)
	// // 		{
	// // 			Ok(_) => println!("should have updated client"),
	// // 			Err(e) => eprintln!("failed to update {}", e),
	// // 		}
	// // 	},
    // //     Err(e) => eprintln!("CLient not found"),
    // // }

	// // let found = db.get_client_name("emet");
	// // match found {
	// // 	Ok(client) => println!("2 Client found {:?}", client),
    // //     Err(e) => eprintln!("CLient not found"),
    // // }

    // HttpServer::new(move || {
    //     App::new()
	// 	.route("/{param1}/{param2}", web::get().to(handler::get_request))
	// 	.default_service(web::route().to(handle_request))
    // })
    // .bind("127.0.0.1:8080")?
    // .run()
    // .await?;

    Ok(())
}

fn setup_database() -> Database
{
	// load .env file into enviroment
	dotenvy::from_filename("usr/src/.env").ok();
	// retrieve the POSTGRES_URL
	let database_url = dotenvy::var("POSTGRES_URL").expect("POSTGRES_URL not set in .env");
	// create new Database
	let db = Database::new(&database_url);

    println!("Database connection established");

	db
}