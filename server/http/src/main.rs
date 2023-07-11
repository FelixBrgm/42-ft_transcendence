#[macro_use]
extern crate diesel;

mod schema;
mod models;
mod ops;
mod db;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use diesel_migrations::run_pending_migrations;
use crate::db::get_connection;
use crate::ops::client_ops;

// GET handler
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, GET request!")
}

// POST handler
#[post("/")]
async fn post_data(data: web::Json<String>) -> impl Responder {
    // Process the data received in the POST request
    // Replace YourStruct with your actual struct for handling the data

    HttpResponse::Ok().body("Hello, POST request!")
}


#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let connection = get_connection();
    run_pending_migrations(&connection)?;
    println!("Database connection established");

    HttpServer::new(move || {
        App::new()
		.service(index)
		.service(post_data)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
