use actix::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::api::game::Stop;
use crate::game::pong;
use crate::game::{ClientMessage, Connect, Disconnect, Socket, UserId};
// use crate::db::Database;
use crate::game::pong::{Player, Pong};

#[derive(Clone)]
pub struct MatchmakingServer {
    queue: Vec<Player>,
    pong_instances: HashMap<(UserId, UserId), Addr<Pong>>,
}

impl MatchmakingServer {
    pub fn new() -> MatchmakingServer {
        println!("MatchmakingServer is up.");
        MatchmakingServer {
            queue: vec![],
            pong_instances: HashMap::new(),
        }
    }

    fn is_player_stored(&self, player_id: UserId) -> bool {
        self.is_player_in_queue(player_id) || self.is_player_in_instance(player_id)
    }

    fn is_player_in_queue(&self, player_id: UserId) -> bool {
        self.queue.iter().any(|player| player.id == player_id)
    }

    fn is_player_in_instance(&self, player_id: UserId) -> bool {
        self.pong_instances
            .keys()
            .any(|(id1, id2)| *id1 == player_id || *id2 == player_id)
    }
}

impl Actor for MatchmakingServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for MatchmakingServer {
    type Result = ();

    fn handle(&mut self, msg: Connect, ctx: &mut Context<Self>) {
        if !self.is_player_stored(msg.id) {
            println!("{} added to the queue", msg.id);
            self.queue.push(Player::new(msg.id, msg.socket, msg.addr));
        }

        if self.queue.len() >= 2 {
            let p1 = self.queue.remove(0);
            let p2 = self.queue.remove(0);
            let player_ids = (p1.id, p2.id);

            println!("starting new game between {:?}", player_ids);
            let pong = Pong::new(
                [p1, p2],
                crate::api::game::GameMode::Matchmaking(ctx.address()),
            )
            .start();

            if !self.is_player_stored(player_ids.0) && !self.is_player_stored(player_ids.1) {
                self.pong_instances.insert(player_ids, pong);
            }
        }
    }
}

impl Handler<Disconnect> for MatchmakingServer {
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
        self.queue.retain(|player| player.id != msg.id);
    }
}

impl Handler<ClientMessage> for MatchmakingServer {
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
