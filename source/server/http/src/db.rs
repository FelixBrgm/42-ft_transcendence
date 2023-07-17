use std::path::Path;
use dotenvy::dotenv;
use lazy_static::lazy_static;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

fn create_db_pool() -> Result<DbPool, Box<dyn std::error::Error>> {

	// Retrieve the database url
    let _ = dotenvy::from_path(Path::new("usr/src/.env"));
    dotenv().ok();
    let database_url =  dotenvy::var("POSTGRES_URL")?;

    // Create a connection manager
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    // Create a connection pool
	Ok(Pool::builder().build(manager)?)
}

// once the lazy static is required it is initiolized
lazy_static! {
    static ref POOL: DbPool = create_db_pool().expect("Failed to create database pool");
}

pub fn get_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
	POOL.get().expect("Failed to get connection")
}

