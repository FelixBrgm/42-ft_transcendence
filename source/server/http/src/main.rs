#[macro_use]
extern crate diesel;

// #![allow(dead_code)]
// #![allow(unused_imports)]
// #[allow(unused_variables)]

mod handler;
mod db;

use db::wrapper::Database;
use dotenvy::dotenv;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

	let db = setup_database();

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
	let db = Database::new(&database_url).expect("Failed to create Database");

    println!("Database connection established");

	db
}