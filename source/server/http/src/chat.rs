use actix::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::db::models::NewMessage;
use crate::db::Database;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<Message>,
    pub room_id: usize,
    pub self_id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
    pub room_id: usize,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub id: usize,
    pub msg: String,
    pub room_id: usize,
}

// --------------------- CHATSERVER ------------------

type Socket = Recipient<Message>;

#[derive (Clone)]
pub struct ChatServer {
	db: Database,
    sessions: HashMap<usize, Socket>,
    rooms: HashMap<usize, HashSet<usize>>,
}

impl ChatServer {
    pub fn new(db: Database) -> ChatServer {
        ChatServer {
			db,
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

		// insert new connectoin into rooms
        self.rooms
            .entry(msg.room_id)
            .or_insert_with(HashSet::new)
            .insert(msg.self_id);
		// update everyobody in the room
        self.rooms
            .get(&msg.room_id)
            .unwrap()
            .iter()
            .filter(|con_id| *con_id.to_owned() != msg.self_id)
            .for_each(|con_id| self.send_message(&format!("{} just joined!", msg.self_id), con_id));
		// add the session
        self.sessions.insert(msg.self_id, msg.addr);
        self.send_message(&format!("your id is {}", &msg.self_id), &msg.self_id);
    }
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("{} disconnected", msg.id);

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

		println!("{:?}", msg);
		match self.db.add_message(&NewMessage{
			sender_id: msg.id as i32,
			room_id: msg.room_id as i32,
			message: msg.msg.to_string(),
		}) {
			Ok(_) => {},
			Err(e) => {println!("{}", e)},
		};

        self.rooms
            .get(&msg.room_id)
            .unwrap()
            .iter()
            .for_each(|client| self.send_message(&msg.msg, client));
    }
}
