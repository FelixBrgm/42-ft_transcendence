
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

	// /// ===============================================================
    // ///                             Client
    // /// ===============================================================

	// // Insert the new client into the clients table
	// pub fn add_client(&self, new_client: &NewClient) -> Result<()> {

    //     diesel::insert_into(clients)
    //         .values(new_client)
    //         .execute(&mut self.pool.get()?)?;
	
    //     Ok(())
    // }

	// // Update the client in the clients table
	// pub fn set_client(&self, new_client: &Client) -> Result<()> {

	// 	  diesel::update(clients.filter(id.eq(new_client.id)))
	// 	  .set(new_client)
	// 	  .execute(&mut self.pool.get()?)?;

	//   	Ok(())
    // }
	
	// // Get the client in the clients table by name
	// pub fn get_client_by_name(&self, name: &str) -> Result<Client> {

	// 	Ok (clients
	// 	.filter(title.eq(name))
	// 	.first::<Client>(&mut self.pool.get()?)?
	// 	)
  	// }

	// // Get the client in the clients table by id
	// pub fn get_client_by_id(&self, find_id: i32) -> Result<Client> {

	// 	Ok (clients
	// 	.find(find_id)
	// 	.first::<Client>(&mut self.pool.get()?)?
	// 	)
  	// }

	// /// ===============================================================
    // ///                             Clients
    // /// ===============================================================

	// pub fn show_clients(&self) -> Result<()> {
	// 	println!("Showing clients...");
	
	// 	let results = clients
	// 	.load::<Client>(&mut self.pool.get()?)
	// 	.unwrap_or(vec![]);
	
	// 	for client in results{
	// 		println!("{:?}", client);
	// 	}

	// 	Ok(())
	// }

	//********************************************************//
	//							Users
	//*********************************************************//
	// Insert the new user into the clients table
	pub fn add_user(&self, new_user: &NewUser) -> Result<()> {
		use super::schema::app_user::dsl::*;
		diesel::insert_into(app_user)
			.values(new_user)
			.execute(&mut self.pool.get()?)?;

		Ok(())
	}

	// Update the client in the clients table
	pub fn update_user(&self, new_user: &UpdateUser) -> Result<()> {
		use super::schema::app_user::dsl::*;
		diesel::update(app_user.filter(id.eq(new_user.id)))
		.set(new_user)
		.execute(&mut self.pool.get()?)?;

		Ok(())
	}

	// Get the client in the clients table by id
	pub fn get_user_by_id(&self, find_id: i32) -> Result<User> {
		use super::schema::app_user::dsl::*;
		Ok (app_user
		.find(find_id)
		.first::<User>(&mut self.pool.get()?)?
		)
	}

	/// ===============================================================
    ///                             USERS
    /// ===============================================================

	pub fn show_users(&self) -> Result<()> {
		println!("Showing all users...");
	
		use super::schema::app_user::dsl::*;
		let results = app_user
		.load::<User>(&mut self.pool.get()?)
		.unwrap_or(vec![]);
	
		for user in results{
			println!("{:?}", user);
		}

		Ok(())
	}


}
