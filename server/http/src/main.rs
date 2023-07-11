#[macro_use]
extern crate diesel;

mod schema;
mod models;
mod ops;
mod db;

use crate::db::setup_database;
use crate::ops::client_ops;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};


#[actix_web::main]
async fn main() ->  Result<(), Box<dyn std::error::Error>> {

	setup_database()?;

	client_ops::insert("fritz");
	client_ops::show();
	println!("find client: {:?}", client_ops::find("fritzi"));
	client_ops::remove("daddy");
	Ok(())
}