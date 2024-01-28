use actix::prelude::*;
use tungstenite::protocol::frame::coding::Data;
use std::collections::HashMap;

use crate::db::Database;
use crate::game::pong;
use crate::game::pong::{Player, Pong};
use crate::game::{ClientMessage, Disconnect, UserId};

use super::OneVsOneConnect;

#[derive(Clone)]
pub struct OneVsOneServer {
	db: Database,
    queue: Vec<(Player, UserId)>,
    pong_instances: HashMap<(UserId, UserId), Addr<Pong>>,
}

impl OneVsOneServer {
    pub fn new(db: Database) -> OneVsOneServer {
        println!("OneVsOneServer is up.");
        OneVsOneServer {
			db,
            queue: vec![],
            pong_instances: HashMap::new(),
        }
    }

    pub fn try_connect(
        &mut self,
        player: Player,
        opponent_uid: UserId,
        ctx: &mut Context<OneVsOneServer>,
    ) {
        let player_id = player.id;
        // Check if Challange already exists
        match self.queue.iter().position(|q| q.1 == player.id) {
            Some(q) => {
                // if yes then start game
                let opponent = self.queue.remove(q);
                println!("starting new 1v1 game");
                let pong = Pong::new(
                    [player, opponent.0],
                    crate::api::game::GameMode::OneVsOne(ctx.address()),
                )
                .start();
                self.pong_instances.insert((player_id, opponent_uid), pong);
            }
            None => {
                // If not add to queye
                self.queue.push((player, opponent_uid));
            }
        }
    }

    // fn is_player_stored(&self, player_id: UserId) -> bool {
    //     self.is_player_in_queue(player_id) || self.is_player_in_instance(player_id)
    // }

    // fn is_player_in_queue(&self, player_id: UserId) -> bool {
    //     self.queue.iter().any(|player| player.0.id == player_id)
    // }

    // fn is_player_in_instance(&self, player_id: UserId) -> bool {
    //     self.pong_instances
    //         .keys()
    //         .any(|(id1, id2)| *id1 == player_id || *id2 == player_id)
    // }
}

impl Actor for OneVsOneServer {
    type Context = Context<Self>;
}

impl Handler<OneVsOneConnect> for OneVsOneServer {
    type Result = ();

    fn handle(&mut self, msg: OneVsOneConnect, ctx: &mut Context<Self>) {
        let player = Player::new(msg.uid, msg.socket, msg.addr);
        self.try_connect(player, msg.opponent, ctx);

        // if !self.is_player_stored(msg.id) {
        //     println!("{} added to the queue", msg.id);
        //     self.queue.push(Player::new(msg.id, msg.socket, msg.addr));
        // }

        // if self.queue.len() >= 2 {
        //     let p1 = self.queue.remove(0);
        //     let p2 = self.queue.remove(0);
        //     let player_ids = (p1.id, p2.id);

        //     println!("starting new game between {:?}", player_ids);
        //     let pong = Pong::new(
        //         [p1, p2],
        //         crate::api::game::GameMode::OneVsOne(ctx.address()),
        //     )
        //     .start();

        //     if !self.is_player_stored(player_ids.0) && !self.is_player_stored(player_ids.1) {
        //         self.pong_instances.insert(player_ids, pong);
        //     }
        // }
    }
}

impl Handler<Disconnect> for OneVsOneServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("{} disconnected", msg.id);

        if let Some((ids, pong)) = self
            .pong_instances
            .iter()
            .find(|(ids, _)| ids.0 == msg.id || ids.1 == msg.id)
        {
            println!("send GameOver from {}", msg.id);
            pong.do_send(pong::GameOver);
        }

        self.pong_instances
            .retain(|ids, _| ids.0 != msg.id && ids.1 != msg.id);
        self.queue.retain(|player| player.0.id != msg.id);
    }
}

impl Handler<ClientMessage> for OneVsOneServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        if let Some((ids, pong)) = self
            .pong_instances
            .iter()
            .find(|(ids, _)| ids.0 == msg.id || ids.1 == msg.id)
        {
            if let Some(c) = msg.msg.chars().last() {
                pong.do_send(pong::PlayerInput { id: msg.id, cmd: c });
            }
        }
    }
}
