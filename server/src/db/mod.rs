mod migrations;
pub mod models;
mod schema;

use anyhow::Result;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use models::*;

type DbConnection =
    diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

#[derive(Clone)]
pub struct Database {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

// TODO NOTES
// finish the friendship anf block function function
// the new functions are untested and remember if you need a transaction, you need to use the same connection

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
        self.update_user(
            &UpdateUser {
                status: Some(status.to_string()),
                ..Default::default()
            },
            id,
        )?;

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

    //     /// ===============================================================
    //     ///                             ROOMS
    //     /// ===============================================================

    // Check if Room exits by id
    pub fn check_room_by_id(&self, room_id: i32) -> Result<bool> {
        use schema::chat_rooms::dsl::*;

        let room_count = chat_rooms
            .filter(id.eq(room_id))
            .count()
            .execute(&mut self.pool.get()?)?;

        Ok(room_count > 0)
    }

    // Check if Room exists by users
    pub fn check_room_by_user(&self, user_1: i32, user_2: i32) -> Result<bool> {
        use schema::chat_rooms::dsl::*;

        let room_count = chat_rooms
            .filter(
                (user1.eq(user_1).and(user2.eq(user_2))).or(user1.eq(user_2).and(user2.eq(user_1))),
            )
            .count()
            .execute(&mut self.pool.get()?)?;

        Ok(room_count > 0)
    }

    // Add a new Room, if it doesn't exists with those users yet and return its id
    pub fn add_room(&self, user_1: i32, user_2: i32) -> Result<i32> {
        use schema::chat_rooms::dsl::*;

        if let Some(existing_id) = self.get_room_by_users(user_1, user_2)? {
            return Ok(existing_id.id);
        }

        let room = NewChatRoom {
            user1: user_1,
            user2: user_2,
        };

        let inserted_id = diesel::insert_into(chat_rooms)
            .values(room)
            .returning(id) // Specify the column you want to retrieve
            .get_result::<i32>(&mut self.pool.get()?)?;

        Ok(inserted_id)
    }

    // Get a room by id
    pub fn get_room_by_id(&self, room_id: i32) -> Result<ChatRoom> {
        use schema::chat_rooms::dsl::*;

        let room = chat_rooms
            .filter(id.eq(room_id))
            .first::<ChatRoom>(&mut self.pool.get()?)
            .map_err(|err| anyhow::anyhow!("Error retrieving chat room: {}", err))?;

        Ok(room)
    }

    // Get a room by id
    pub fn get_rooms_by_uid(&self, uid: i32) -> Result<Vec<ChatRoom>> {
        use schema::chat_rooms::dsl::*;

        let rooms = chat_rooms
            .filter(user1.eq(uid).or(user2.eq(uid)))
            .load::<ChatRoom>(&mut self.pool.get()?)
            .map_err(|err| anyhow::anyhow!("Error retrieving chat room: {}", err))?;

        Ok(rooms)
    }

    // Get a room by it's users
    pub fn get_room_by_users(&self, user_id_1: i32, user_id_2: i32) -> Result<Option<ChatRoom>> {
        use schema::chat_rooms::dsl::*;

        let room = chat_rooms
            .filter(
                (user1.eq(user_id_1).and(user2.eq(user_id_2)))
                    .or(user1.eq(user_id_2).and(user2.eq(user_id_1))),
            )
            .first::<ChatRoom>(&mut self.pool.get()?)
            .optional()
            .map_err(|err| anyhow::anyhow!("Error retrieving chat room: {}", err))?;

        Ok(room)
    }

    // Remove the room from the chat_rooms table by id
    pub fn remove_room(&self, room_id: i32) -> Result<()> {
        use schema::chat_rooms::dsl::*;

        diesel::delete(chat_rooms.filter(id.eq(room_id))).execute(&mut self.pool.get()?)?;

        Ok(())
    }

    // / ===============================================================
    // /                            FRIENDS
    // / ===============================================================

    // Checks if a friendship exists
    pub fn check_friendship(&self, user_id: i32, friend_id: i32) -> Result<bool> {
        use schema::friend_ship::dsl::*;

        let fs = friend_ship
            .filter(
                user1
                    .eq(user_id)
                    .and(user2.eq(friend_id))
                    .or(user1.eq(friend_id).and(user2.eq(user_id))),
            )
            .first::<Friendship>(&mut self.pool.get()?)
            .optional()?;

        Ok(fs.is_some())
    }

    // Creates a new friendship if it doesn't exist
    pub fn create_friendship(&self, user_id: i32, friend_id: i32) -> Result<i32> {
        use schema::friend_ship::dsl::*;

        if self.check_friendship(user_id, friend_id)? {
            return Err(anyhow::anyhow!(
                "Friendship already exists between {} and {}!",
                user_id,
                friend_id
            ));
        }

        let fs = NewFriendship {
            user1: user_id,
            user2: friend_id,
        };

        let inserted_id = diesel::insert_into(friend_ship)
            .values(fs)
            .returning(id)
            .get_result::<i32>(&mut self.pool.get()?)?;

        Ok(inserted_id)
    }

    // Removes a friendship
    pub fn remove_friendship(&self, user_id: i32, friend_id: i32) -> Result<()> {
        use schema::friend_ship::dsl::*;

        if !self.check_friendship(user_id, friend_id)? {
            return Err(anyhow::anyhow!(
                "No friendship exists between {} and {}!",
                user_id,
                friend_id
            ));
        }

        diesel::delete(
            friend_ship.filter(
                user1
                    .eq(user_id)
                    .and(user2.eq(friend_id))
                    .or(user1.eq(friend_id).and(user2.eq(user_id))),
            ),
        )
        .execute(&mut self.pool.get()?)?;
        Ok(())
    }

    pub fn get_all_friendships(&self, user_id: i32) -> Result<Vec<(i32, i32, i32)>> {
        use schema::friend_ship::dsl::*;

        let all_users = friend_ship
            .filter(user1.eq(user_id).or(user2.eq(user_id)))
            .load(&mut self.pool.get()?)?;

        return Ok(all_users);
    }

    // / ===============================================================
    // /                            BLOCKED
    // / ===============================================================

    // Checks if user is blocked
    pub fn check_blocked(&self, uid: i32, blocked_uid: i32) -> Result<bool> {
        use schema::blocked_users::dsl::*;

        let b = blocked_users
            .filter(user_id.eq(uid).and(blocked_user_id.eq(blocked_uid)))
            .first::<Blocked>(&mut self.pool.get()?)
            .optional()?;

        Ok(b.is_some())
    }

    // Creates a new blocked if it doesn't exist
    pub fn create_blocked(&self, uid: i32, blocked_uid: i32) -> Result<i32> {
        use schema::blocked_users::dsl::*;

        if self.check_blocked(uid, blocked_uid)? {
            return Err(anyhow::anyhow!(
                "{} is already blocked by {}!",
                blocked_uid,
                uid
            ));
        }

        let b = NewBlocked {
            user_id: uid,
            blocked_user_id: blocked_uid,
        };

        let inserted_id = diesel::insert_into(blocked_users)
            .values(b)
            .returning(id)
            .get_result::<i32>(&mut self.pool.get()?)?;

        Ok(inserted_id)
    }

    // Removes a blocked
    pub fn remove_blocked(&self, uid: i32, blocked_uid: i32) -> Result<()> {
        use schema::blocked_users::dsl::*;

        if !self.check_blocked(uid, blocked_uid)? {
            return Err(anyhow::anyhow!(
                "{} is not blocked by {}!",
                blocked_uid,
                uid
            ));
        }

        diesel::delete(blocked_users.filter(user_id.eq(uid).and(blocked_user_id.eq(blocked_uid))))
            .execute(&mut self.pool.get()?)?;

        Ok(())
    }

    // / ===============================================================
    // /                            MESSAGES
    // / ===============================================================

    // Insert new Message
    pub fn add_message(&self, msg: &NewMessage) -> Result<i32> {
        use schema::chat_messages::dsl::*;

        let inserted_id = diesel::insert_into(chat_messages)
            .values(msg)
            .returning(id)
            .get_result::<i32>(&mut self.pool.get()?)?;

        Ok(inserted_id)
    }

    // get all messages of a room
    pub fn get_messages_by_room_id(&self, rid: i32) -> Result<Vec<Message>> {
        use schema::chat_messages::dsl::*;

        let room_messages = chat_messages
            .filter(room_id.eq(rid))
            .load::<Message>(&mut self.pool.get()?)?;

        Ok(room_messages)
    }

    // / ===============================================================
    // /                            GAMES
    // / ===============================================================

    pub fn add_game(&self, winner_uid: i32, looser_uid: i32) -> Result<i32> {
        use schema::game_match::dsl::*;

        let g = NewGameMatch {
            winner: winner_uid,
            looser: looser_uid,
        };

        let inserted_id = diesel::insert_into(game_match)
            .values(g)
            .returning(id)
            .get_result::<i32>(&mut self.pool.get()?)?;

        Ok(inserted_id)
    }

    pub fn get_games_by_uid(&self, uid: i32) -> Result<Vec<GameMatch>> {
        use schema::game_match::dsl::*;

        let games = game_match
            .filter(winner.eq(uid).or(looser.eq(uid)))
            .load(&mut self.pool.get()?)?;

        Ok(games)
    }

    // DEBUG

    pub fn get_all_users(&self) -> Result<Vec<User>> {
        use schema::app_user::dsl::*;

        let all_users = app_user.load::<User>(&mut self.pool.get()?)?;

        Ok(all_users)
    }

    pub fn get_all_blocked(&self) -> Result<Vec<Blocked>> {
        use schema::blocked_users::dsl::*;

        let all_users = blocked_users.load::<Blocked>(&mut self.pool.get()?)?;

        Ok(all_users)
    }
}
