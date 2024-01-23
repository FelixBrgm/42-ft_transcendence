use actix::prelude::*;
use log::{error, info};
use rand::seq::index;
use std::collections::{HashMap, HashSet};

use crate::db::models::NewMessage;
use crate::db::Database;

type Socket = Recipient<ChatMessage>;
type UserId = usize;

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

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub id: UserId,
    pub msg: String,
}

// --------------------- CHATSERVER ------------------

#[derive(Clone)]
pub struct ChatServer {
    db: Database,
    sockets: HashMap<UserId, Socket>,
    rooms: HashMap<usize, (UserId, UserId)>,
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

    fn send_message(&self, message: &str, recipient: &usize) {
        if let Some(socket_recipient) = self.sockets.get(recipient) {
            let _ = socket_recipient.do_send(ChatMessage(message.to_owned()));
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
        println!("{} joined", msg.id);

        self.sockets.insert(msg.id, msg.addr);
        self.send_message(&format!("you just connected"), &msg.id);
    }
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("{} disconnected", msg.id);

        if self.sockets.remove(&msg.id).is_some() {
        //     self.rooms
        //         .get(&msg.room_id)
        //         .unwrap()
        //         .iter()
        //         .filter(|conn_id| *conn_id.to_owned() != msg.id)
        //         .for_each(|user_id| {
        //             self.send_message(&format!("{} disconnected.", &msg.id), user_id)
        //         });
        //     if let Some(chatserver) = self.rooms.get_mut(&msg.room_id) {
        //         if chatserver.len() > 1 {
        //             chatserver.remove(&msg.id);
        //         } else {
        //             self.rooms.remove(&msg.room_id);
        //         }
        //     }
        }
    }
}

// TODO: make a good error handling
impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        println!("{:?}", msg);

        let index = msg.msg.find(':');
        if index.is_none() {
            println!("Delimiter not found in the string.");
            return;
        }
        let index = index.unwrap();

        let (first_part, second_part) = msg.msg.split_at(index);

        let recipient_id = first_part.parse::<usize>();
        if recipient_id.is_err() {
            println!("Recipient id is not valid.");
            return;
        }
        let recipient_id = recipient_id.unwrap();

        if self.rooms.iter().find(|room| {
            room.1 .0 == recipient_id && room.1 .1 == msg.id
                || room.1 .0 == msg.id && room.1 .1 == recipient_id
        }).is_some() {
            self.send_message(second_part, &recipient_id);
        };
    }
}
