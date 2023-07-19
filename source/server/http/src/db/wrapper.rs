
use super::migrations;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct Database
{
    pub pool:  Pool<ConnectionManager<PgConnection>>,
}

impl Database
{
	pub fn new(database_url: &str) -> Result<Self, Box<dyn std::error::Error>>
	{
		// Create a connection manager for PostgreSQL
		let manager = ConnectionManager::<PgConnection>::new(database_url);
		
		// Create a new connection pool
		let pool = Pool::new(manager)?;

		// Run pending migrations
        migrations::run_migrations(database_url).expect("Failed to run migrations.");

		Ok(Database { pool })
	}
}

// fn create_db_pool() -> Result<DbPool, Box<dyn std::error::Error>> {


// }

// // once the lazy static is required it is initiolized
// lazy_static! {
//     static ref POOL: DbPool = create_db_pool().expect("Failed to create database pool");
// }

// pub fn get_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
// 	POOL.get().expect("Failed to get connection")
// }

