use std::path::Path;
use dotenvy::dotenv;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

type DbPool = Pool<ConnectionManager<PgConnection>>;

fn create_db_pool() -> Result<DbPool, Box<dyn std::error::Error>> {

	// retrieve the database url
    dotenvy::from_path(Path::new("../../database/.env"))?;
    dotenv().ok();
    let database_url =  dotenvy::var("POSTGRES_URL")?;

    // Create a connection manager
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    // Create a connection pool
	Ok(Pool::builder().build(manager)?)
}

#[actix_web::main]
async fn main() ->  Result<(), Box<dyn std::error::Error>> {

	let pool: DbPool = create_db_pool()?;


	Ok(())

}