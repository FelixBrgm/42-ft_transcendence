mod ball;
mod config;
mod player;
// mod utils;

use std::time::{Duration, Instant};
use actix::prelude::*;

pub use crate::game::{Socket, Message};
pub use self::config::GameConfig;
pub use self::player::Player;
pub use self::ball::Ball;


#[derive(Message)]
#[rtype(result = "()")]
pub struct Tick;

#[derive(Message)]
#[rtype(result = "()")]
pub struct PlayerInput {
    pub player_id: usize,
    pub cmd: char,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct GameOver;

#[derive(Debug, Clone)]
pub struct Pong {
    players: [Player; 2],
    score: [u8; 2],
    ball: Ball,
    config: GameConfig,
    finished: bool,
}

impl Actor for Pong {
    type Context = Context<Self>;

	fn started(&mut self, ctx: &mut Self::Context) {

		self.send_to_players(Message("BEG".to_owned()));
		self.tick(ctx);
    }

}

impl Pong {
	pub fn new(players: [Player; 2]) -> Pong {
        Pong {
            players,
            score: [0; 2],
            ball: Ball::new(),
            config: GameConfig::new(),
            finished: false,
        }
	}

	fn send_to_players(&self, msg: Message) {
        for player in &self.players {
            let _ = player.socket.do_send(msg.clone());
        }
    } 

	fn tick(&self, ctx: &mut Context<Self>) {
        const TICK_INTERVAL: Duration = Duration::from_millis(100);
		if (self.finished) {
			return;
		}
		
        ctx.run_later(TICK_INTERVAL, |_, ctx| {
			ctx.address().do_send(Tick);
        });
	}
}

impl Handler<Tick> for Pong {
    type Result = ();

    fn handle(&mut self, _: Tick, ctx: &mut Self::Context) {
		// println!("tick: should send out information for player");
		self.tick(ctx);
    }
}

// maybe call it no connection
impl Handler<GameOver> for Pong {
	type Result = ();

	fn handle(&mut self, _: GameOver, ctx: &mut Self::Context) {
		println!("GameOver");
		self.finished = true;
		self.send_to_players(Message("END".to_owned()));
	}
}

impl Handler<PlayerInput> for Pong {
	type Result = ();

	fn handle(&mut self, input: PlayerInput, ctx: &mut Self::Context) {
		if self.players[0].id == input.player_id {
			self.players[0].last_input = input.cmd;
		}
		else {
			self.players[1].last_input  = input.cmd;
		}
	}
}


