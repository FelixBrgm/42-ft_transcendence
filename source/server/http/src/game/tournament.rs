use actix::prelude::*;
use log::{error, info};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::db::models::NewMessage;
use crate::db::Database;
use crate::game::pong;
use crate::game::pong::{Player, Pong};
use crate::game::{ClientMessage, Connect, Disconnect, Socket, UserId};

use super::{Create, TournamentConnect};

#[derive(Clone)]
struct Match {
    player1: usize,
    player2: usize,
    winner: Option<usize>,
    instance: Addr<Pong>,
}

impl Match {
    pub fn new(player1: usize, player2: usize, instance: Addr<Pong>) -> Self {
        Match {
            player1,
            player2,
            winner: None,
            instance,
        }
    }
}

#[derive(Clone)]
struct Round {
    matches: Vec<Match>,
}

impl Round {
    pub fn new() -> Self {
        Round { matches: vec![] }
    }
}

#[derive(Clone)]
pub struct Tournament {
    uid: UserId,
    size: u8,
    players: Vec<Player>,
    rounds: Vec<Round>,
}

impl Tournament {
    pub fn new(uid: UserId, size: u8) -> Self {
        Tournament {
            uid,
            size,
            players: vec![],
            rounds: vec![],
        }
    }

    pub fn add_player(&mut self, player: Player) {
        println!("{} added to the tournament", player.id);
        if let None = self.players.iter().find(|p| p.id == player.id) {
            self.players.push(player);
        }
        if self.players.len() == self.size as usize {
            self.start();
        }
    }

    pub fn start(&mut self) {
        println!("starting tournament");
    }

    pub fn start_round(&mut self) {
        println!("starting round");
        let mut round = Round::new();
        if self.rounds.len() == 0 {
            while self.players.len() > 0 {
                let p1 = self.players.remove(0);
                let p2 = self.players.remove(0);
                let player_ids = (p1.id, p2.id);

                println!("starting new game between {:?}", player_ids);
                let pong = Pong::new([p1, p2]).start();

                let m = Match::new(player_ids.0, player_ids.1, pong);
                round.matches.push(m);
            }
            
        } else {
        }
    }
}

#[derive(Clone)]
pub struct TournamentServer {
    tournaments: HashMap<usize, Tournament>,
}

// implement TournamentServer logic -> what information do i need from the requesting client
impl TournamentServer {
    pub fn new() -> TournamentServer {
        println!("TournamentServer is up.");
        TournamentServer {
            tournaments: HashMap::new(),
        }
    }
}

impl Actor for TournamentServer {
    type Context = Context<Self>;
}

impl Handler<TournamentConnect> for TournamentServer {
    type Result = ();

    fn handle(&mut self, msg: TournamentConnect, _: &mut Context<Self>) {
        if let Some(t) = self.tournaments.get_mut(&msg.tournament_id) {
            t.add_player(Player::new(msg.uid, msg.socket));
        }
    }
}

impl Handler<Create> for TournamentServer {
    type Result = ();

    fn handle(&mut self, msg: Create, _: &mut Context<Self>) {
        let tournament = Tournament::new(msg.id, msg.size);
        self.tournaments.insert(msg.id, tournament);
    }
}

impl Handler<Disconnect> for TournamentServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {}
}

impl Handler<ClientMessage> for TournamentServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        println!("tournament message: {}", msg.msg);
        // if let Some((ids, pong)) = self
        //     .pong_instances
        //     .iter()
        //     .find(|(ids, _)| ids.0 == msg.id || ids.1 == msg.id)
        // {
        //     if let Some(c) = msg.msg.chars().last() {
        //         pong.do_send(pong::PlayerInput { id: msg.id, cmd: c });
        //     }
        // }
    }
}
