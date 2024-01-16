use actix::Addr;
use std::collections::HashMap;

pub mod actor;
pub mod server;

pub type UserId = i32;
pub enum SocketManagerMessage {
    RegisterUser {
        uid: UserId,
        addr: Addr<actor::WsActor>,
    },
    UnregisterUser {
        uid: UserId,
    },
}

pub struct SocketManager {
    user_sockets: HashMap<UserId, Addr<actor::WsActor>>,
}

impl SocketManager {
    pub fn new() -> SocketManager {
        SocketManager {
            user_sockets: HashMap::new(),
        }
    }
}
