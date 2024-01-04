use actix::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::db::models::NewMessage;
use crate::db::Database;

/// Chat server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// New chat session is created
#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<Message>,
    pub room_id: usize,
    pub self_id: usize,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
    pub room_id: usize,
}

// TO DO: i might want to derive the Message trait for a NewMessage instead
// / Send message to specific room
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    /// Id of the client session
    pub id: usize,
    /// Peer message
    pub msg: String,
    /// Room name
    pub room_id: usize,
}

// --------------------- CHATSERVER ------------------

type Socket = Recipient<Message>;

#[derive(Debug, Clone)]
pub struct ChatServer {
    sessions: HashMap<usize, Socket>,
    rooms: HashMap<usize, HashSet<usize>>,
}

impl ChatServer {
    pub fn new() -> ChatServer {
        println!("chat server is being created.");
        ChatServer {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
        }
    }

    fn send_message(&self, message: &str, recipient: &usize) {
        if let Some(socket_recipient) = self.sessions.get(recipient) {
            let _ = socket_recipient.do_send(Message(message.to_owned()));
        } else {
            println!(
                "attempting to send message to {}, but couldn't find him in the session",
                recipient
            )
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
        println!("Someone joined");

        self.rooms
            .entry(msg.room_id)
            .or_insert_with(HashSet::new)
            .insert(msg.self_id);
        self.rooms
            .get(&msg.room_id)
            .unwrap()
            .iter()
            .filter(|con_id| *con_id.to_owned() != msg.self_id)
            .for_each(|con_id| self.send_message(&format!("{} just joined!", msg.self_id), con_id));
        self.sessions.insert(msg.self_id, msg.addr);
        self.send_message(&format!("your id is {}", &msg.self_id), &msg.self_id);
    }
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("Someone disconnected");

        if self.sessions.remove(&msg.id).is_some() {
            self.rooms
                .get(&msg.room_id)
                .unwrap()
                .iter()
                .filter(|conn_id| *conn_id.to_owned() != msg.id)
                .for_each(|user_id| {
                    self.send_message(&format!("{} disconnected.", &msg.id), user_id)
                });
            if let Some(chatserver) = self.rooms.get_mut(&msg.room_id) {
                if chatserver.len() > 1 {
                    chatserver.remove(&msg.id);
                } else {
                    self.rooms.remove(&msg.room_id);
                }
            }
        }
    }
}

impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        self.rooms
            .get(&msg.room_id)
            .unwrap()
            .iter()
            .for_each(|client| self.send_message(&msg.msg, client));
    }
}
