// #[macro_use]
// extern crate diesel;

// #![allow(dead_code)]
// #![allow(unused_imports)]
// #[allow(unused_variables)]

mod db;

use db::wrapper::Database;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};


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

// Define a handler function for a specific route
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix Web!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

	let db = setup_database();

	db.show_clients();

	println!("unes");

       // Start the Actix Web server
	HttpServer::new(|| {
			App::new()
			.route("/", web::get().to(hello))
    })
    .bind("127.0.0.1:8080")? // Bind the server to localhost on port 8080
    .run()
    .await
}