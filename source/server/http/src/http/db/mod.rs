pub mod models;
mod schema;
mod migrations;

use models::*;
use anyhow::Result;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

#[derive(Clone)]
pub struct Database {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl Database {
    pub fn new() -> Self {
        let database_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL not set in .env");

        // Create a connection manager for PostgreSQL
        let manager = ConnectionManager::<PgConnection>::new(&database_url);

        // Create a new connection pool
        let pool = Pool::new(manager).expect("Failed to connect to database.");

        // Run pending migrations
        migrations::run_migrations(&database_url).expect("Failed to run migrations.");

        Database { pool }
    }

    //********************************************************//
    //							Users
    //*********************************************************//
    // Insert the new user into the users table
    pub fn add_user(&self, new_user: &NewUser) -> Result<()> {
        use schema::app_user::dsl::*;
        diesel::insert_into(app_user)
            .values(new_user)
            .execute(&mut self.pool.get()?)?;

        Ok(())
    }

    // Update the user in the users table
    pub fn update_user(&self, new_user: &UpdateUser) -> Result<()> {
        use schema::app_user::dsl::*;
        diesel::update(app_user.filter(id.eq(new_user.id)))
            .set(new_user)
            .execute(&mut self.pool.get()?)?;

        Ok(())
    }

	// Update the user status in the users table
	pub fn update_user_status(&self, id: i32, status: &str) -> Result<()> {

		self.update_user(&UpdateUser{
			id,
			status: Some(status.to_string()),
			..Default::default()
		})?;

		Ok(())
	}

    // Get the user in the users table by id
    pub fn get_user_by_id(&self, find_id: i32) -> Result<User> {
        use schema::app_user::dsl::*;
        Ok(app_user
            .find(find_id)
            .first::<User>(&mut self.pool.get()?)?)
    }

    /// ===============================================================
    ///                             USERS
    /// ===============================================================

	/// Get a list of all users from the users table
	pub fn get_users(&self) -> Result<Vec<User>> {
		use schema::app_user::dsl::*;
		Ok(app_user.load(&mut self.pool.get()?)?)
	}

    pub fn show_users(&self) -> Result<()> {
        println!("Showing all users...");

        use schema::app_user::dsl::*;
        let results = app_user
            .load::<User>(&mut self.pool.get()?)
            .unwrap_or(vec![]);

        for user in results {
            println!("{:?}", user);
        }

        Ok(())
    }
}
