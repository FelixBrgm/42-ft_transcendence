use crate::api::game::Stop;
use crate::game::pong::{Player, Pong};
use crate::game::Message;
use crate::game::{ClientMessage, Disconnect, UserId};
use actix::prelude::*;
use num_traits::pow;
use std::collections::HashMap;

use crate::db::Database;
use super::pong::GameFinished;
use super::{Create, TournamentConnect};

#[derive(Clone, Debug)]
struct Match {
    player1: usize,
    player2: usize,
    pub winner: Option<usize>,
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

#[derive(Clone, Debug)]
struct Round {
    matches: Vec<Match>,
}

impl Round {
    pub fn new() -> Self {
        Round { matches: vec![] }
    }
}

#[derive(Clone, Debug)]
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

    pub fn try_connect(&mut self, player: Player, ctx: &mut Context<TournamentServer>) {
        // When games are running

        if self.rounds.len() == 0 {
            if let None = self.players.iter().find(|p| p.id == player.id) {
                self.players.push(player);
            }
        } else {
            let current_round = self.rounds.last().unwrap();
            if current_round.matches.iter().any(|m| m.winner == None) {
                player.addr.do_send(Stop { id: player.id });
                return;
            } else {
                let player_doesnt_exists =
                    self.players.iter().find(|p| p.id == player.id).is_none();
                let won_round = current_round
                    .matches
                    .iter()
                    .any(|m| m.winner == Some(player.id));

                if player_doesnt_exists && won_round {
                    self.players.push(player);
                } else {
                    player.addr.do_send(Stop { id: player.id });
                    return;
                }
            }
        }

        let needed_size = self.size / pow(2, self.rounds.len());
        if self.players.len() == needed_size as usize {
            self.start_round(ctx);
        }
    }

    pub fn start_round(&mut self, ctx: &mut Context<TournamentServer>) {
        println!("starting round");
        let mut round = Round::new();

        while self.players.len() > 0 {
            let p1 = self.players.remove(0);
            let p2 = self.players.remove(0);
            let player_ids = (p1.id, p2.id);

            p1.addr
                .do_send(Message(format!("RND {}", self.rounds.len() + 1).to_owned()));
            p1.addr.do_send(Message(
                format!("SZE {}", self.size / pow(2, self.rounds.len())).to_owned(),
            ));
            p2.addr
                .do_send(Message(format!("RND {}", self.rounds.len() + 1).to_owned()));
            p2.addr.do_send(Message(
                format!("SZE {}", self.size / pow(2, self.rounds.len())).to_owned(),
            ));

            println!("starting new game between {:?}", player_ids);
            let pong = Pong::new(
                [p1, p2],
                crate::api::game::GameMode::Tournament(ctx.address()),
            )
            .start();

            let m = Match::new(player_ids.0, player_ids.1, pong);
            round.matches.push(m);
        }

        self.rounds.push(round);
    }
}

#[derive(Clone)]
pub struct TournamentServer {
	db: Database,
    tournaments: HashMap<usize, Tournament>,
}

// implement TournamentServer logic -> what information do i need from the requesting client
impl TournamentServer {
    pub fn new(db: Database) -> TournamentServer {
        println!("TournamentServer is up.");
        TournamentServer {
			db,
            tournaments: HashMap::new(),
        }
    }
}

impl Handler<GameFinished> for TournamentServer {
    type Result = ();

    fn handle(&mut self, msg: GameFinished, ctx: &mut Context<Self>) {
        // After game is finished
        println!("{:?}", msg);

        let tournament = self
            .tournaments
            .iter_mut()
            .find(|t| {
                t.1.rounds
                    .iter()
                    .last()
                    .unwrap()
                    .matches
                    .iter()
                    .any(|m| m.player1 == msg.players[0] || m.player2 == msg.players[0])
            })
            .unwrap();

        let tournament = tournament.1;

        let m = tournament
            .rounds
            .last_mut()
            .unwrap()
            .matches
            .iter_mut()
            .find(|m| m.player1 == msg.players[0] || m.player2 == msg.players[0])
            .unwrap();

        m.winner = Some(msg.winner);
    }
}

impl Actor for TournamentServer {
    type Context = Context<Self>;
}

impl Handler<TournamentConnect> for TournamentServer {
    type Result = ();

    fn handle(&mut self, msg: TournamentConnect, ctx: &mut Context<Self>) {
        if let Some(t) = self.tournaments.get_mut(&msg.tournament_id) {
            t.try_connect(Player::new(msg.uid, msg.socket, msg.addr), ctx);
        }
    }
}

impl Handler<Create> for TournamentServer {
    type Result = ();

    fn handle(&mut self, msg: Create, ctx: &mut Context<Self>) {
        println!(
            "Tournament created with id {} and a size of {}.",
            msg.id, msg.size
        );
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
