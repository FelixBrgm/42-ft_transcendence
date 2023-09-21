mod migrations;
pub mod models;
mod schema;

use anyhow::Result;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use models::*;

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
    pub fn update_user(&self, user: &UpdateUser, uid: i32) -> Result<()> {
        use schema::app_user::dsl::*;
        diesel::update(app_user.filter(id.eq(uid)))
            .set(user)
            .execute(&mut self.pool.get()?)?;

        Ok(())
    }

    // Update the user status in the users table
    pub fn update_user_status(&self, id: i32, status: &str) -> Result<()> {
        self.update_user(&UpdateUser {
            status: Some(status.to_string()),
            ..Default::default()
        }, id)?;

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
    pub fn remove_user(&self, user_id: i32) -> Result<()> {
        use schema::app_user::dsl::*;

        diesel::delete(app_user.filter(id.eq(user_id))).execute(&mut self.pool.get()?)?;

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

    // Insert the new room into the chat_rooms table, returns the id of the inserted room
    pub fn add_room(&self, room: &NewChatRoom) -> Result<i32> {
        use schema::chat_rooms::dsl::*;

        let inserted_id = diesel::insert_into(chat_rooms)
            .values(room)
            .returning(id) // Specify the column you want to retrieve
            .get_result::<i32>(&mut self.pool.get()?)?;

        Ok(inserted_id)
    }

    // creates a room and sets the owver in the connection tables, returns the id of the created room
    pub fn create_room(&self, uid: i32, name: &str, public: bool) -> Result<i32> {
        let new_room = NewChatRoom {
            owner: uid,
            name: String::from(name),
            topic: None,
            is_public: public,
            password: None,
        };

        let rid = self.add_room(&new_room)?;
        self.add_connection(uid, rid)?;
        Ok(rid)
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

    // Get the rooms in the chat_rooms table by name
    pub fn get_rooms_by_name(&self, room_name: &str) -> Result<Vec<ChatRoom>> {
        use schema::chat_rooms::dsl::*;

        let matching_rooms = chat_rooms
            .filter(name.eq(room_name))
            .load::<ChatRoom>(&mut self.pool.get()?)?;

        Ok(matching_rooms)
    }

    // Get the rooms id's in the chat_rooms table by name
    pub fn get_room_ids_by_name(&self, room_name: &str) -> Result<Vec<i32>> {
        use schema::chat_rooms::dsl::*;

        let room_ids = chat_rooms
            .filter(name.eq(room_name))
            .select(id)
            .load::<i32>(&mut self.pool.get()?)?;

        Ok(room_ids)
    }

    // Remove the room from the chat_rooms table by id
    pub fn remove_room(&self, room_id: i32) -> Result<()> {
        use schema::chat_rooms::dsl::*;

        diesel::delete(chat_rooms.filter(id.eq(room_id))).execute(&mut self.pool.get()?)?;

        Ok(())
    }

    /// ===============================================================
    ///                        CONNECTIONS
    /// ===============================================================

    // checks if a connectino between user and room exits
    pub fn check_connection(&self, uid: i32, rid: i32) -> Result<bool> {
        use schema::user_room_connection::dsl as user_room;

        let user_room_exists = user_room::user_room_connection
            .filter(user_room::user_id.eq(uid).and(user_room::room_id.eq(rid)))
            .count()
            .first::<i64>(&mut self.pool.get()?)?;

        Ok(user_room_exists > 0)
    }

    // adds a conneciton between user and room
    pub fn add_connection(&self, uid: i32, rid: i32) -> Result<()> {
        use schema::user_room_connection::dsl as user_room;

        if self.check_connection(uid, rid)? == true {
            return Ok(());
        }

        let user_room_con = UserRoomConnection {
            user_id: uid,
            room_id: rid,
        };

        diesel::insert_into(user_room::user_room_connection)
            .values(user_room_con)
            .on_conflict_do_nothing()
            .execute(&mut self.pool.get()?)?;

        Ok(())
    }

    // gets all the rooms a user is in
    pub fn get_user_connections(&self, uid: i32) -> Result<Vec<ChatRoom>> {
        use schema::chat_rooms::dsl::chat_rooms;
        use schema::user_room_connection::dsl::*;

        let user_connections = user_room_connection
            .filter(user_id.eq(uid))
            .inner_join(chat_rooms)
            .select(chat_rooms::all_columns())
            .load::<ChatRoom>(&mut self.pool.get()?)?;

        Ok(user_connections)
    }

    // gets all the users a room has
    pub fn get_room_connections(&self, rid: i32) -> Result<Vec<User>> {
        use schema::app_user::dsl::app_user;
        use schema::user_room_connection::dsl::*;

        let room_connections = user_room_connection
            .filter(room_id.eq(rid))
            .inner_join(app_user)
            .select(app_user::all_columns())
            .load::<User>(&mut self.pool.get()?)?;

        Ok(room_connections)
    }

    // removes the connection between user and room
    pub fn remove_connection(&self, uid: i32, rid: i32) -> Result<()> {
        use schema::user_room_connection::dsl as user_room;

        diesel::delete(
            user_room::user_room_connection
                .filter(user_room::user_id.eq(uid).and(user_room::room_id.eq(rid))),
        )
        .execute(&mut self.pool.get()?)?;
        Ok(())
    }

    /// ===============================================================
    ///                            MESSAGES
    /// ===============================================================

    // Insert new Message
    pub fn add_message(&self, msg: &NewMessage) -> Result<i32> {
        use schema::chat_messages::dsl::*;

        let inserted_id = diesel::insert_into(chat_messages)
            .values(msg)
            .returning(id)
            .get_result::<i32>(&mut self.pool.get()?)?;

        Ok(inserted_id)
    }

    pub fn get_message_by_room_id(&self, rid: i32) -> Result<Vec<Message>> {
        use schema::chat_messages::dsl::*;

        let room_messages = chat_messages
            .filter(room_id.eq(rid))
            .load::<Message>(&mut self.pool.get()?)?;

        Ok(room_messages)
    }

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

    /// Get a list of all connections from the chat_rooms table
    pub fn get_connections(&self) -> Result<Vec<UserRoomQuery>> {
        use schema::user_room_connection::dsl as user_room;

        Ok(user_room::user_room_connection.load::<UserRoomQuery>(&mut self.pool.get()?)?)
    }

    /// Get a list of all messagess from the chat_rooms table
    pub fn get_messages(&self) -> Result<Vec<Message>> {
        use schema::chat_messages::dsl as user_room;

        Ok(user_room::chat_messages.load::<Message>(&mut self.pool.get()?)?)
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

    /// ===============================================================
    ///                             CLEAR
    /// ===============================================================

    // DCear all content from the app_user table
    pub fn clear_app_user_table(&self) -> Result<()> {
        use schema::app_user::dsl::*;

        diesel::delete(app_user).execute(&mut self.pool.get()?)?;
        Ok(())
    }

    // Clear all content from the chat_rooms table
    pub fn clear_chat_rooms_table(&self) -> Result<()> {
        use schema::chat_rooms::dsl::*;

        diesel::delete(chat_rooms).execute(&mut self.pool.get()?)?;
        Ok(())
    }

    // Clear all content from the chat_emssages table
    pub fn clear_chat_messages(&self) -> Result<()> {
        use schema::chat_messages::dsl::*;

        diesel::delete(chat_messages).execute(&mut self.pool.get()?)?;
        Ok(())
    }

    // Clear all content from the user_room_connection table
    pub fn clear_user_room_connection(&self) -> Result<()> {
        use schema::user_room_connection::dsl::*;

        diesel::delete(user_room_connection).execute(&mut self.pool.get()?)?;
        Ok(())
    }

    // Clear all the tables of the db
    pub fn clear_tables(&self) -> Result<()> {
        self.clear_chat_messages()?;
        self.clear_user_room_connection()?;
        self.clear_chat_rooms_table()?;
        self.clear_app_user_table()?;
        Ok(())
    }
}

// messages should only be able to be added when the connection to the room exists
#[cfg(test)]
mod testing {

    use super::*;
    // #[should_panic]
    // #[ignore]

    #[test]
    fn clear_tables() -> Result<()> {
        use schema::app_user::dsl::app_user;
        use schema::chat_rooms::dsl::chat_rooms;
        use schema::user_room_connection::dsl::user_room_connection;

        let db = Database::new();

        db.clear_tables()?;
        assert_eq!(Ok(0), app_user.count().first::<i64>(&mut db.pool.get()?));
        assert_eq!(Ok(0), chat_rooms.count().first::<i64>(&mut db.pool.get()?));
        assert_eq!(
            Ok(0),
            user_room_connection
                .count()
                .first::<i64>(&mut db.pool.get()?)
        );

        Ok(())
    }

    #[test]
    fn user_fn() -> Result<()> {
        use schema::app_user::dsl::*;

        let db = Database::new();
        let mut con = db.pool.get()?;

        con.transaction::<_, anyhow::Error, _>(
            |con: &mut diesel::r2d2::PooledConnection<ConnectionManager<PgConnection>>| {
                db.clear_tables()?;

                let new_user = NewUser {
                    id: 1,
                    login: String::from("test_user"),
                    avatar: String::from("some"),
                };

                let update_user = models::UpdateUser {
                    login: Some(String::from("updated_user")),
                    avatar: None,
                    password: None,
                    status: None,
                    wins: Some(1),
                    losses: None,
                };

                db.add_user(&new_user)?;
                assert_eq!(Ok(1), app_user.count().first::<i64>(con));

                db.update_user(&update_user, 1)?;
                assert_eq!(Ok(1), app_user.count().first::<i64>(con));
                assert_eq!(
                    "updated_user",
                    app_user.filter(id.eq(1)).first::<User>(con)?.login
                );

                db.update_user_status(1, "test")?;
                assert_eq!(Ok(1), app_user.count().first::<i64>(con));
                assert_eq!("test", app_user.filter(id.eq(1)).first::<User>(con)?.status);

                db.remove_user(2)?;
                assert_eq!(Ok(1), app_user.count().first::<i64>(con));

                db.remove_user(1)?;
                assert_eq!(Ok(0), app_user.count().first::<i64>(con));

                Ok(())
            },
        )?;

        Ok(())
    }

    #[test]
    fn room_fn() -> Result<()> {
        use schema::chat_rooms::dsl::*;

        let db = Database::new();

        let mut con = db.pool.get()?;

        con.transaction::<_, anyhow::Error, _>(
            |con: &mut diesel::r2d2::PooledConnection<ConnectionManager<PgConnection>>| {
                db.clear_tables()?;

                let new_user = NewUser {
                    id: 1,
                    login: String::from("test_user"),
                    avatar: String::from("some"),
                };

                let new_room = NewChatRoom {
                    owner: 1,
                    name: String::from("Chatroom name"),
                    topic: None,
                    is_public: false,
                    password: None,
                };

                db.add_user(&new_user)?;
                assert_eq!(
                    Ok(1),
                    schema::app_user::dsl::app_user.count().first::<i64>(con)
                );

                let rid = db.add_room(&new_room)?;
                assert_eq!(Ok(1), chat_rooms.count().first::<i64>(con));

                let update_room = models::UpdateChatRoom {
                    name: Some(String::from("Update ChatRoom name")),
                    id: rid,
                    is_public: Some(true),
                    topic: None,
                    password: None,
                };

                db.update_room(&update_room)?;
                assert_eq!(Ok(1), chat_rooms.count().first::<i64>(con));
                assert_eq!(
                    "Update ChatRoom name",
                    chat_rooms.filter(id.eq(rid)).first::<ChatRoom>(con)?.name
                );

                db.remove_room(rid + 1)?;
                assert_eq!(Ok(1), chat_rooms.count().first::<i64>(con));

                db.remove_room(rid)?;
                assert_eq!(Ok(0), chat_rooms.count().first::<i64>(con));

                Ok(())
            },
        )?;

        Ok(())
    }

    #[test]
    fn connection_fn() -> Result<()> {
        let db = Database::new();

        let mut con = db.pool.get()?;

        con.transaction::<_, anyhow::Error, _>(|con| {
            db.clear_tables()?;

            let new_user = NewUser {
                id: 1,
                login: String::from("test_user"),
                avatar: String::from("some"),
            };

            let new_room = NewChatRoom {
                owner: 1,
                name: String::from("Chatroom name"),
                topic: None,
                is_public: false,
                password: None,
            };

            db.add_user(&new_user)?;
            assert_eq!(
                Ok(1),
                schema::app_user::dsl::app_user.count().first::<i64>(con)
            );

            let rid = db.add_room(&new_room)?;
            assert_eq!(
                Ok(1),
                schema::chat_rooms::dsl::chat_rooms
                    .count()
                    .first::<i64>(con)
            );

            assert!(db.check_connection(1, rid)? == false);

            db.add_connection(1, rid)?;
            assert!(db.check_connection(1, rid)?);
            assert!(db.check_connection(1, rid + 1)? == false);

            db.remove_connection(1, rid)?;
            assert!(db.check_connection(1, rid)? == false);
            Ok(())
        })?;

        Ok(())
    }

    #[test]
    fn get_connection_fn() -> Result<()> {
        let db = Database::new();

        let mut con = db.pool.get()?;

        con.transaction::<_, anyhow::Error, _>(|con| {
            db.clear_tables()?;

            let new_user = NewUser {
                id: 1,
                login: String::from("test_user"),
                avatar: String::from("some"),
            };

            let new_user_2 = NewUser {
                id: 2,
                login: String::from("test_user 2"),
                avatar: String::from("some"),
            };
            db.add_user(&new_user)?;
            assert_eq!(
                Ok(1),
                schema::app_user::dsl::app_user.count().first::<i64>(con)
            );
            db.add_user(&new_user_2)?;
            assert_eq!(
                Ok(2),
                schema::app_user::dsl::app_user.count().first::<i64>(con)
            );

            let one = db.create_room(1, "first_test", false)?;
            let sec = db.create_room(1, "second_test", false)?;
            db.create_room(1, "third_test", false)?;
            db.create_room(2, "fourth_test", false)?;

            assert_eq!(3, db.get_user_connections(1)?.len());
            assert_eq!(1, db.get_user_connections(2)?.len());

            assert_eq!(1, db.get_room_connections(one)?.len());
            db.add_connection(2, sec)?;
            assert_eq!(2, db.get_room_connections(sec)?.len());

            Ok(())
        })?;

        Ok(())
    }

    #[test]
    fn add_message_to_chat() -> Result<()> {
        let db = Database::new();

        let mut con = db.pool.get()?;

        con.transaction::<_, anyhow::Error, _>(|con| {
            db.clear_tables()?;

            let new_user = NewUser {
                id: 1,
                login: String::from("test_user"),
                avatar: String::from("some"),
            };

            let new_user_2 = NewUser {
                id: 2,
                login: String::from("test_user 2"),
                avatar: String::from("some"),
            };

            let user2_id = db.add_user(&new_user)?;
            assert_eq!(
                Ok(1),
                schema::app_user::dsl::app_user.count().first::<i64>(con)
            );
            let user2_id = db.add_user(&new_user_2)?;
            assert_eq!(
                Ok(2),
                schema::app_user::dsl::app_user.count().first::<i64>(con)
            );

            let room = db.create_room(1, "testroom", false)?;
            // db.add_connection(2, room)?;
            assert_eq!(1, db.get_user_connections(1)?.len());
            assert_eq!(0, db.get_user_connections(2)?.len());

            let message = NewMessage {
                room_id: room,
                sender_id: 1,
                message: String::from("rofl"),
            };

            db.add_message(&message)?;
            assert_eq!(1, db.get_messages()?.len());

            let msg = db.get_message_by_room_id(room)?;
            println!("Message: {:?}", msg);
            assert_eq!(msg[0].message, "rofl");
            Ok(())
        })?;
        Ok(())
    }
}
