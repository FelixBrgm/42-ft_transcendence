
use std::env;
use tokio_postgres::{Config, NoTls, Client, Connection, Socket};
use tokio_postgres::tls::NoTlsStream;

type DataBaseConnection = (Client, Connection<Socket, NoTlsStream>);

async fn connect() -> Result<DataBaseConnection, Box<dyn std::error::Error>>
{
	// Retrieve environment variables for connection details
	let host = env::var("POSTGRES_HOST").expect("POSTGRES_HOST not set");
	let port = env::var("POSTGRES_PORT").expect("POSTGRES_PORT not set");
	let user = env::var("POSTGRES_USER").expect("POSTGRES_USER not set");
	let password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD not set");
	let dbname = env::var("POSTGRES_DB").expect("POSTGRES_DB not set");

	// Create a new database configuration
	let mut config = Config::new();
	config.host(&host);
	config.port(port.parse::<u16>().expect("Invalid POSTGRES_PORT"));
	config.user(&user);
	config.password(&password);
	config.dbname(&dbname);

	// Establish a connection to the database
	let connection:DataBaseConnection  = config.connect(NoTls).await?;
	Ok(connection)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
  
	let (client, connection) = connect().await?;

    // Spawn a task to process the connection in the background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

	println!("works:)");

    Ok(())
}
