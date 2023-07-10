use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};


#[actix_web::main]
async fn main() ->  std::io::Result<()> {
    dotenvy::dotenv().ok();

    // Create a connection manager
    let database_url = "postgres://theOperator:IHaveAllThePower@localhost/postgres_db";
    let manager = ConnectionManager::<PgConnection>::new(database_url);

	println!("1");
	
    // Create a connection pool
    let pool = Pool::builder()
	.build(manager)
	.expect("Failed to create database pool");
	
	println!("1");
	Ok(())

}