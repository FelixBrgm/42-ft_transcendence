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
    pub input: char,
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
    last_tick_time: Instant,
    time_since_last_tick: Duration,
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
            last_tick_time: Instant::now(),
            time_since_last_tick: Duration::from_secs(0),
            finished: false,
        }
	}

	fn send_to_players(&self, msg: Message) {
        for player in &self.players {
            let _ = player.socket.do_send(msg.clone());
        }
    } 

	fn tick(&self, ctx: &mut Context<Self>) {
        const TICK_INTERVAL: Duration = Duration::from_secs(2); // Adjust as needed
        ctx.run_interval(TICK_INTERVAL, |actor, ctx| {


			println!("---");
			ctx.address().do_send(Tick);
        });
	}
}

impl Handler<Tick> for Pong {
    type Result = ();

    fn handle(&mut self, _: Tick, ctx: &mut Self::Context) {
		println!("tick: should send out information for player");
    }
}

impl Handler<GameOver> for Pong {
	type Result = ();

	fn handle(&mut self, _: GameOver, ctx: &mut Self::Context) {

		self.send_to_players(Message("END".to_owned()));
	}
}


