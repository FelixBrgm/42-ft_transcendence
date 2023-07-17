// #[macro_use]
// extern crate diesel;

mod handler;
mod db;

use actix_web::{web, App, HttpServer};
use diesel_migrations::run_pending_migrations;
use crate::db::get_connection;
use crate::handler::handle_request;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let connection = get_connection();
    run_pending_migrations(&connection)?;
    println!("Database connection established");

	

    HttpServer::new(move || {
        App::new()
		.route("/{param1}/{param2}", web::get().to(handler::get_request))
		.default_service(web::route().to(handle_request))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}