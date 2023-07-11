#[macro_use]
extern crate diesel;

mod schema;
mod models;
mod ops;
mod db;

use crate::db::setup_database;
use crate::ops::client_ops::create_client;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};


use crate::db::get_connection;

#[actix_web::main]
async fn main() ->  Result<(), Box<dyn std::error::Error>> {

	setup_database()?;

	create_client("peter");
	create_client("peter");
	create_client("peter");
	create_client("peter");

	Ok(())
}