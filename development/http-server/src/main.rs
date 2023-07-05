
use std::env;
use bb8::Pool;
use bb8_postgres::tokio_postgres::NoTls;
use bb8_postgres::PostgresConnectionManager;
// use tokio_postgres::connect;

type DataBasePool = Pool<PostgresConnectionManager<NoTls>>;
type DataBaseConnection<'a> = bb8::PooledConnection<'a, PostgresConnectionManager<NoTls>>;

// Connect to database && create Pool
async fn create_pool() -> Result<DataBasePool, Box<dyn std::error::Error>>
{
	// Retrieve environment variables for connection details
	let host = env::var("POSTGRES_HOST").expect("POSTGRES_HOST not set");
	let port = env::var("POSTGRES_PORT").expect("POSTGRES_PORT not set");
	let user = env::var("POSTGRES_USER").expect("POSTGRES_USER not set");
	let password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD not set");
	let dbname = env::var("POSTGRES_DB").expect("POSTGRES_DB not set");

	// Create a connection string
    let connection_string = format!(
        "postgres://{}:{}@{}:{}/{}",
        user, password, host, port, dbname
    );

	// Create a connection manager
    let manager = PostgresConnectionManager::new_from_stringlike(connection_string, NoTls)?;
	
	// Create a connection pool
    let pool:DataBasePool = bb8::Pool::builder()
        .build(manager)
        .await?;

    Ok(pool)
}

async fn create_table(pool: &DataBasePool, name: &str) -> Result<(), Box<dyn std::error::Error>>
{
	let	connection: DataBaseConnection = pool.get().await?;

	// Create a connection string
    let create_string = format!(
        "CREATE TABLE IF NOT EXISTS {} (id SERIAL PRIMARY KEY, name TEXT NOT NULL, age INT NOT NULL)",
        name
    );

	connection
	.execute(&create_string, &[])
	.await?;

	// Release the connection back to the pool
	drop(connection);

	Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
	// get_pool
	let	pool: DataBasePool = create_pool().await?;

	create_table(&pool, "clients").await?;
	create_table(&pool, "others").await?;

	let	connection: DataBaseConnection = pool.get().await?;

    // Spawn a task to process the connection in the background
    tokio::spawn(async move {
		let _ = connection;
    });
	
    println!("Connected to the database and acquired a connection");

    Ok(())
}

