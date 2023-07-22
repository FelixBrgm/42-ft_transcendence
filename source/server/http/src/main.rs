// #[macro_use]
// extern crate diesel;

#![allow(dead_code)]
#![allow(unused_imports)]
#[allow(unused_variables)]

mod db;
mod api;

use db::wrapper::Database;
use actix_web::{web, App, HttpResponse, HttpServer, HttpRequest, Responder};
use actix_web::middleware::Logger;
use actix_web::{cookie::Key};
use actix_identity::IdentityMiddleware;
// use actix_session::{storage::RedisSessionStore, SessionMiddleware};

use crate::db::models::NewClient;

fn setup_database() -> Database
{
	// load .env file into enviroment
	dotenvy::from_filename("usr/src/.env").ok();
	// retrieve the POSTGRES_URL
	let database_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL not set in .env");
	// create new Database
	let db = Database::new(&database_url);

    println!("Database connection established");

	db
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	// std::env::set_var("RUST_LOG", "debug");
	// std::env::set_var("RUST_BACKTRACE", "1");
	// env_logger::init();

	let db = setup_database();

    // Start the Actix Web server
	HttpServer::new( move || {

			// cookie::key
			// auth client
			// cors

			App::new()
			.app_data(web::Data::new(db.clone()))
			.wrap(IdentityMiddleware::default())
			.wrap(Logger::default())
			.service(
				web::resource("/health")
				.route(web::get().to(|| async { HttpResponse::Ok().json("I am alive!")})),
			)
			.service(
				web::scope("/api")
				.configure(api::client::init)
			)
	})
    .bind("127.0.0.1:8080") // Bind the server to localhost on port 8080
	.expect("Failed to bind to port 8080")
    .run()
    .await
}

// i want to add a rickroll with external_resource in app