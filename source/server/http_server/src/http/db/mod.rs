mod migrations;
pub mod models;
mod schema;

use anyhow::Result;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use models::*;

use crate::http::db::schema::user_room_connection;

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

    // Check if user exists
    pub fn check_user(&self, user_id: i32) -> Result<bool> {
        use schema::app_user::dsl::*;

        let user_count = app_user
            .filter(id.eq(user_id))
            .count()
            .execute(&mut self.pool.get()?)?;

        Ok(user_count > 0)
    }

    // Insert the new user into the users table
    pub fn add_user(&self, user: &NewUser) -> Result<()> {
        use schema::app_user::dsl::*;
        diesel::insert_into(app_user)
            .values(user)
            .execute(&mut self.pool.get()?)?;

        Ok(())
    }

    // Update the user in the users table
    pub fn update_user(&self, user: &UpdateUser) -> Result<()> {
        use schema::app_user::dsl::*;
        diesel::update(app_user.filter(id.eq(user.id)))
            .set(user)
            .execute(&mut self.pool.get()?)?;

        Ok(())
    }

    // Update the user status in the users table
    pub fn update_user_status(&self, id: i32, status: &str) -> Result<()> {
        self.update_user(&UpdateUser {
            id,
            status: Some(status.to_string()),
            ..Default::default()
        })?;

        Ok(())
    }

    // Get the user in the users table by id
    pub fn get_user_by_id(&self, user_id: i32) -> Result<Option<User>> {
        use schema::app_user::dsl::*;

        match self.check_user(user_id)? {
            true => Ok(Some(
                app_user
                    .find(user_id)
                    .first::<User>(&mut self.pool.get()?)?,
            )),
            false => Ok(None),
        }
    }

    // Remove the user from the users table by id
    pub fn remove_user(&self, id: i32) -> Result<()> {
        use schema::app_user::dsl::*;

        diesel::delete(app_user.filter(id.eq(id))).execute(&mut self.pool.get()?)?;

        Ok(())
    }

    /// ===============================================================
    ///                             ROOMS
    /// ===============================================================

    // Check if Room exits
    pub fn check_room(&self, room_id: i32) -> Result<bool> {
        use schema::chat_rooms::dsl::*;

        let room_count = chat_rooms
            .filter(id.eq(room_id))
            .count()
            .execute(&mut self.pool.get()?)?;

        Ok(room_count > 0)
    }

    // Insert the new room into the chat_rooms table
    pub fn add_room(&self, room: &NewChatRoom) -> Result<()> {
        use schema::chat_rooms::dsl::*;

        diesel::insert_into(chat_rooms)
            .values(room)
            .execute(&mut self.pool.get()?)?;

        Ok(())
    }

    // Update the room in the chat_rooms table
    pub fn update_room(&self, room: &UpdateChatRoom) -> Result<()> {
        use schema::chat_rooms::dsl::*;
        diesel::update(chat_rooms.filter(id.eq(room.id)))
            .set(room)
            .execute(&mut self.pool.get()?)?;

        Ok(())
    }

    // Get the room in the chat_rooms table by id
    pub fn get_room_by_id(&self, room_id: i32) -> Result<Option<ChatRoom>> {
        use schema::chat_rooms::dsl::*;

        match self.check_room(room_id)? {
            true => Ok(Some(
                chat_rooms
                    .find(room_id)
                    .first::<ChatRoom>(&mut self.pool.get()?)?,
            )),
            false => Ok(None),
        }
    }

    // Remove the room from the chat_rooms table by id
    pub fn remove_room(&self, id: i32) -> Result<()> {
        use schema::chat_rooms::dsl::*;

        diesel::delete(chat_rooms.filter(id.eq(id))).execute(&mut self.pool.get()?)?;

        Ok(())
    }

	/// ===============================================================
    ///                        CONNECTIONS
    /// ===============================================================

    fn add_room_user(&self, uid: i32, rid: i32) -> Result<()> {
        use schema::room_user_connection::dsl::*;

        let con = RoomUserConnection {
            user_id: uid,
            room_id: rid,
        };

        diesel::insert_into(room_user_connection)
            .values(con)
            .execute(&mut self.pool.get()?)?;
        Ok(())
    }

    fn add_user_room(&self, uid: i32, rid: i32) -> Result<()> {
        use schema::user_room_connection::dsl::*;

        let con = UserRoomConnection {
            user_id: uid,
            room_id: rid,
        };

        diesel::insert_into(user_room_connection)
            .values(con)
            .execute(&mut self.pool.get()?)?;
        Ok(())
    }

    pub fn add_connection(&self, user_id: i32, room_id: i32) -> Result<()> {
        self.add_room_user(user_id, room_id)?;
        self.add_user_room(user_id, room_id)?;
        Ok(())
    }

    // pub fn rem_connection(&self)


	// pub fn get_user_room(&self) -> Result<Vec<UserRoomConnection>> {
	// 	use schema::user_room_connection::dsl::*;
	// 	Ok(user_room_connection.load(&mut self.pool.get()?)?)
	// }

	// pub fn get_room_user(&self) -> Result<Vec<RoomUserConnection>> {
	// 	use schema::room_user_connection::dsl::*;
	// 	Ok(room_user_connection.load(&mut self.pool.get()?)?)
	// }

	// pub fn get_connecions(&self) -> Result<(Vec<RoomUserConnection>, Vec<UserRoomConnection>)> {
	// 	let con = &mut self.pool.get()?;
	// 	Ok((self.get_room_user()?, self.get_user_room()?))
	// }
    /// ===============================================================
    ///                             GET ALL
    /// ===============================================================

    /// Get a list of all users from the users table
    pub fn get_users(&self) -> Result<Vec<User>> {
        use schema::app_user::dsl::*;
        Ok(app_user.load(&mut self.pool.get()?)?)
    }

    /// Get a list of all rooms from the chat_rooms table
    pub fn get_rooms(&self) -> Result<Vec<ChatRoom>> {
        use schema::chat_rooms::dsl::*;
        Ok(chat_rooms.load(&mut self.pool.get()?)?)
    }


    /// ===============================================================
    ///                             DEBUG
    /// ===============================================================

    pub fn show_rooms(&self) -> Result<()> {
        println!("Showing all rooms...");

        use schema::chat_rooms::dsl::*;
        let results = chat_rooms
            .load::<ChatRoom>(&mut self.pool.get()?)
            .unwrap_or(vec![]);

        for user in results {
            println!("{:?}", user);
        }

        Ok(())
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

// maybe implement some db tests
