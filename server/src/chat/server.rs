use actix::prelude::*;
use log::{error, info};
use rand::seq::index;
use std::collections::{HashMap, HashSet};

use crate::db::models::NewMessage;
use crate::db::Database;
use crate::chat::UserId;

type Socket = Recipient<ChatMessage>;

#[derive(Message)]
#[rtype(result = "()")]
pub struct ChatMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub id: UserId,
    pub addr: Socket,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
	pub id: UserId,
}

#[derive(Message)]
#[rtype(result ="()")]
pub struct InsertRoom {
	pub room_id: i32,
    pub user1: UserId,
    pub user2: UserId,
}

#[derive(Message)]
#[rtype(result ="()")]
pub struct BlockUser {
    pub user_id: UserId,
    pub blocked_id: UserId,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub id: UserId,
    pub msg: String,
}

// --------------------- CHATSERVER ------------------

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Pair {
    user1: UserId,
	user2: UserId,
}

impl Pair {
	pub fn new(user1: UserId, user2: UserId) -> Pair {
		let (min, max) = if user1 < user2 { (user1, user2) } else { (user2, user1) };
        Pair { user1: min, user2: max }
	} 
}


#[derive(Clone)]
pub struct ChatServer {
    db: Database,
    sockets: HashMap<UserId, Socket>,
	rooms: HashMap<Pair, i32>,
}

impl ChatServer {
    pub fn new(db: Database) -> ChatServer {
        println!("ChatServer is up.");
        ChatServer {
            db,
            sockets: HashMap::new(),
            rooms: HashMap::new(),
        }
    }

    fn send_message(&self, message: &str, recipient: &i32) {
        if let Some(socket_recipient) = self.sockets.get(recipient) {
            let _ = socket_recipient.do_send(ChatMessage(message.to_owned()));
        } else {
            println!(
                "attempting to send message to {}, but couldn't find him in the session",
                recipient
            )
        }
    }

	fn parse_message(&self, msg: String) -> Option<(i32, String)> {

		let Some(index) = msg.find(':') else {
			println!("Delimiter not found in the string.");
			return None;
		};

        let (first_part, second_part) = msg.split_at(index);

		let Ok(recipient_id) = first_part.parse::<i32>() else {
			println!("Recipient id is not valid.");
			return None;
		};

		Some((recipient_id, second_part.trim_start_matches(':').to_string()))
	}
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
        println!("{} joined", msg.id);

        self.sockets.insert(msg.id, msg.addr);
        self.send_message(&format!("you just connected"), &msg.id);
    }
}

// might need to remove the rooms that have neither active users
impl Handler<Disconnect> for ChatServer {
    type Result = ();

	// let all people know that you disconnected
    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("{} disconnected", msg.id);
        self.sockets.remove(&msg.id);
    }
}


impl Handler<InsertRoom> for ChatServer {
	type Result = ();

    fn handle(&mut self, room: InsertRoom, _: &mut Context<Self>) {
		self.rooms.insert(Pair::new(room.user1, room.user2), room.room_id);
		dbg!(&self.rooms);
    }
}

impl Handler<BlockUser> for ChatServer {
	type Result = ();

    fn handle(&mut self, block: BlockUser, _: &mut Context<Self>) {
		self.rooms.remove(&Pair::new(block.user_id, block.blocked_id));
		dbg!(&self.rooms);
    }
}

impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {

        let Some((recipient_id, text)) = self.parse_message(msg.msg) else {
			self.send_message("the format was invalid", &msg.id);
			return;
		};

		if let Some(rid) = self.rooms.get(&Pair::new(msg.id, recipient_id)) {
			
			self.send_message(&text, &recipient_id);

			let _ = self.db.add_message(&NewMessage{
				sender_id: msg.id as i32,
				room_id: *rid,
				message: text,
			});
		};
    }
}
