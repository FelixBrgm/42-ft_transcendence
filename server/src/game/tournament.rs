use super::{Create, TournamentConnect};
use crate::game::actor::Stop;

use crate::game::actor::GameMode;
use crate::game::pong::{Player, PlayerInput, Pong, RoundResult};
use crate::game::Message;
use crate::game::{ClientMessage, Disconnect, UserId};

use actix::prelude::*;
use num_traits::pow;
use std::collections::HashMap;

use crate::db::Database;

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

#[derive(Clone)]
pub struct Tournament {
    db: Database,
    uid: UserId,
    size: u8,
    players: Vec<Player>,
    rounds: Vec<Round>,
}

impl Tournament {
    pub fn new(uid: UserId, size: u8, db: Database) -> Self {
        Tournament {
            db,
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
        }
        // else {
        //     let current_round = self.rounds.last().unwrap();
        //     if current_round.matches.iter().any(|m| m.winner == None) {
        //         player.addr.do_send(Stop { id: player.id });
        //         return;
        //     } else {
        //         let player_doesnt_exists =
        //             self.players.iter().find(|p| p.id == player.id).is_none();
        //         let won_round = current_round
        //             .matches
        //             .iter()
        //             .any(|m| m.winner == Some(player.id));

        //         if player_doesnt_exists && won_round {
        //             self.players.push(player);
        //         } else {
        //             player.addr.do_send(Stop { id: player.id });
        //             return;
        //         }
        //     }
        // }

        let needed_size = self.size / pow(2, self.rounds.len());
        if self.players.len() == needed_size as usize {
            self.start_round(ctx);
        }
    }

    pub fn start_round(&mut self, ctx: &mut Context<TournamentServer>) {
        println!("Starting round");
        let mut round = Round::new();

        // send the MATCHES in each round
        for participant in &self.players {
            for matching in self.players.chunks(2) {
                if let [player1, player2] = matching {
                    let match_msg = format!("MATCH {} {}", player1.id, player2.id);
                    participant.addr.do_send(Message(match_msg));
                }
            }
        }

        while self.players.len() > 1 {
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
            let pong = Pong::new([p1, p2], GameMode::Tournament(ctx.address()), self.db.clone()).start();

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

impl Handler<RoundResult> for TournamentServer {
    type Result = ();

    fn handle(&mut self, msg: RoundResult, ctx: &mut Context<Self>) {
        let tournament = self
            .tournaments
            .iter_mut()
            .find(|t| {
                if t.1.rounds.len() == 0 {
                    false
                } else {
                    t.1.rounds
                        .iter()
                        .last()
                        .unwrap()
                        .matches
                        .iter()
                        .any(|m| m.player1 == msg.winner.id || m.player2 == msg.winner.id)
                }
            })
            .unwrap();

        let tournament = tournament.1;

        let m = tournament
            .rounds
            .last_mut()
            .unwrap()
            .matches
            .iter_mut()
            .find(|m| m.player1 == msg.winner.id || m.player2 == msg.winner.id)
            .unwrap();

        // if msg.winner.id as i32 != msg.looser as i32 {
        //     // let _ = self.db.insert_game(msg.winner.id as i32, msg.looser as i32);
        // }

        m.winner = Some(msg.winner.id);

        dbg!(&msg.winner);
        // add winner to players
        tournament.players.push(msg.winner);

        let required_amount = tournament.size as usize / (tournament.rounds.len() + 1);
        println!("required amount {}", required_amount);
        if required_amount == 1 && tournament.players.len() == 1 {
            let player = &tournament.players[0];
            player.addr.do_send(Stop { id: player.id });
        // self.tournaments.remove(&tournament.uid);
        } else if tournament.players.len() == required_amount {
            tournament.start_round(ctx);
        }
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
        } else {
            msg.addr.do_send(Stop { id: msg.uid });
        }
    }
}

impl Handler<Create> for TournamentServer {
    type Result = ();

    fn handle(&mut self, msg: Create, _: &mut Context<Self>) {
        let tournament = Tournament::new(msg.id, msg.size, self.db.clone());
        if let None = self.tournaments.insert(msg.id, tournament) {
            println!(
                "Tournament created with id {} and a size of {}.",
                msg.id, msg.size
            );
        }
    }
}

impl Handler<Disconnect> for TournamentServer {
    type Result = ();

    fn handle(&mut self, _msg: Disconnect, _: &mut Context<Self>) {}
}

impl Handler<ClientMessage> for TournamentServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        println!("tournament message: {}", msg.msg);

        if let Some((_, tournament)) = self.tournaments.iter().find(|(_, t)| {
            t.rounds.last().map_or(false, |last_round| {
                last_round
                    .matches
                    .iter()
                    .any(|m| m.player1 == msg.id || m.player2 == msg.id)
            })
        }) {
            if let Some(last_round) = tournament.rounds.last() {
                if let Some(m) = last_round
                    .matches
                    .iter()
                    .find(|m| m.player1 == msg.id || m.player2 == msg.id)
                {
                    if let Some(c) = msg.msg.chars().last() {
                        m.instance.do_send(PlayerInput { id: msg.id, cmd: c })
                    }
                }
            }
        }
    }
}
