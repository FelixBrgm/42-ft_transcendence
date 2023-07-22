
use super::schema::clients::dsl::*;
use super::migrations;
use super::models::*;
use anyhow::Result;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};


#[derive(Clone)]
pub struct Database
{
    pub pool:  Pool<ConnectionManager<PgConnection>>,
}

impl Database
{
	pub fn new(database_url: &str) -> Self
	{
		// Create a connection manager for PostgreSQL
		let manager = ConnectionManager::<PgConnection>::new(database_url);
		
		// Create a new connection pool
		let pool = Pool::new(manager).expect("Failed to connect to database.");

		// Run pending migrations
        migrations::run_migrations(database_url).expect("Failed to run migrations.");

		Database { pool }
	}

	/// ===============================================================
    ///                             Client
    /// ===============================================================

	// Insert the new client into the clients table
	pub fn add_client(&self, new_client: &NewClient) -> Result<()> {

        diesel::insert_into(clients)
            .values(new_client)
            .execute(&mut self.pool.get()?)?;
	
        Ok(())
    }

	// Update the client in the clients table
	pub fn set_client(&self, new_client: &Client) -> Result<()> {

		  diesel::update(clients.filter(id.eq(new_client.id)))
		  .set(new_client)
		  .execute(&mut self.pool.get()?)?;

	  	Ok(())
    }
	
	// Get the client in the clients table by name
	pub fn get_client_name(&self, name: &str) -> Result<Client> {

		Ok (clients
		.filter(title.eq(name))
		.first::<Client>(&mut self.pool.get()?)?
		)
  	}

	// Get the client in the clients table by id
	pub fn get_client_id(&self, find_id: i32) -> Result<Client> {

		Ok (clients
		.find(find_id)
		.first::<Client>(&mut self.pool.get()?)?
		)
  	}

	/// ===============================================================
    ///                             Clients
    /// ===============================================================

	pub fn show_clients(&self) -> Result<()> {
		println!("Showing clients...");
	
		let results = clients
		.load::<Client>(&mut self.pool.get()?)
		.unwrap_or(vec![]);
	
		for client in results{
			println!("{:?}", client);
		}

		Ok(())
	}
}
