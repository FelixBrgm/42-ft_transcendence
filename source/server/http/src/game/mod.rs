use actix::prelude::*;
use log::{error, info};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod pong;

use crate::db::models::NewMessage;
use crate::db::Database;
use crate::game::pong::{Pong, Player};


pub type Socket = Recipient<Message>;
type UserId = usize;

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub id: UserId,
    pub socket: Socket,
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
    pub msg: char,
}

// -------- GameServer

#[derive(Clone)]
pub struct GameServer {
    queue: Vec<Player>,
	pong_instances: HashMap<(UserId, UserId), Addr<Pong>>,
}

impl GameServer {
    pub fn new() -> GameServer {
        println!("GameServer is up.");
        GameServer { queue: vec![],
		pong_instances: HashMap::new(), }
    }
}

impl Actor for GameServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for GameServer {
	type Result = ();

	fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
		println!("{} added to the queue", msg.id);

		self.queue.push(Player::new(msg.id, msg.socket));
		
		if self.queue.len() >= 2 {

			let p1 = self.queue.remove(0);
            let p2 = self.queue.remove(0);
			let player_ids = (p1.id, p2.id);

			println!("starting new game between {:?}", player_ids);
			let pong = Pong::new([p1, p2]).start();

			self.pong_instances.insert(player_ids, pong);

		}
	}
}

impl Handler<Disconnect> for GameServer {
	type Result = ();

	fn handle(&mut self, msg: Disconnect, _:&mut Context<Self>) {
		println!("{} disconnected", msg.id);


		dbg!(&self.pong_instances);
		self.pong_instances
		.iter()
		.filter(|(ids, _)| ids.0 == msg.id || ids.1 == msg.id)
		.for_each(|(_, pong)| {println!("i get here");  pong.do_send(pong::GameOver)});

		self.queue.retain(|player| player.id != msg.id);
	} 
}


impl Handler<ClientMessage> for GameServer {
	type Result = ();

	fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
		println!("{} send this: {}", msg.id, msg.msg);
	}
}
