
use actix::prelude::*;
use std::collections::HashMap;

use crate::db::models::NewMessage;
use crate::db::Database;

/// Chat server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// New chat session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

// TO DO: i might want to derive the Message trait for a NewMessage instead
/// Send message to specific room
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    /// Id of the client session
    pub id: usize,
    /// Peer message
    pub msg: String,
    /// Room name
    pub room: String,
}

#[derive(Debug, Clone)]
pub struct ChatServer {
	sessions: HashMap<usize, Recipient<Message>>,
}

impl ChatServer {
    pub fn new() -> ChatServer {
        ChatServer{
			sessions: HashMap::new(),
		}
    }
}

impl Actor for ChatServer {
	    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors
	type Context = Context<Self>;
}

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Connect> for ChatServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> usize {
        println!("Someone joined");

        // // notify all users in same room
        // self.send_message("main", "Someone joined", 0);

        // // register session with random id
        // let id = self.rng.gen::<usize>();
        // self.sessions.insert(id, msg.addr);

        // // auto join session to main room
        // self.rooms.entry("main".to_owned()).or_default().insert(id);

        // let count = self.visitor_count.fetch_add(1, Ordering::SeqCst);
        // self.send_message("main", &format!("Total visitors {count}"), 0);

        // // send id back
        // id
		0
    }
}

impl Handler<ClientMessage> for ChatServer {
	type Result = ();

	fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
		println!("should send message");
	}
}