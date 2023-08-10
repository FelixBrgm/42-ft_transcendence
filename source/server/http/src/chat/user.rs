
use super::bridge;
use bridge::Connection;

pub struct User {
    pub uid: String,
    pub connection: Connection, // added stuff like blocked
}

impl User {
    pub fn new(uid: String, connection: Connection) -> Self {
        User { uid, connection }
    }

    pub fn is_disconnected(&mut self) -> bool {
        self.connection.is_disconnected()
    }
}
