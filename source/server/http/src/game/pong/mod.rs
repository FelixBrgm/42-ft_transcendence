mod ball;
mod config;
mod player;
// mod utils;

use actix::prelude::*;
use std::time::{Duration, Instant};

pub use self::ball::Ball;
pub use self::config::GameConfig;
pub use self::player::Player;
pub use crate::game::{Message, Socket};

const TICK_INTERVAL: Duration = Duration::from_millis(100);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Tick;

#[derive(Message)]
#[rtype(result = "()")]
pub struct PlayerInput {
    pub id: usize,
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
	last_tick_time: Instant,
    finished: bool,
}

impl Pong {
    pub fn new(players: [Player; 2]) -> Pong {
        Pong {
            players,
            score: [0; 2],
            ball: Ball::new(),
            config: GameConfig::new(),
			last_tick_time: Instant::now(),
            finished: false,
        }
    }

    fn send_to_players(&self, msg: Message) {
        for player in &self.players {
            let _ = player.socket.do_send(msg.clone());
        }
    }

	fn send_pos(&mut self) {
		// weirdly doesn't sleep -> find out if it updates the struct
		let msg = format!(
            "POS {:05} {:05} {:05} {:05}",
            self.players[0].position, self.players[1].position, self.ball.x, self.ball.y
        );
		self.send_to_players(Message(msg));
	}

	fn update(&mut self) {

		self.ball.update(100, &self.config, &self.players, &mut self.score);

		for player in self.players.iter_mut() {
            player.update(100, &self.config);
        }

	}

    fn tick(&mut self, ctx: &mut Context<Self>) {
        if self.finished {
            return;
        }

		self.update();
		self.send_pos();

        ctx.run_later(TICK_INTERVAL, |_, ctx| {
            ctx.address().do_send(Tick);
        });
    }
}

impl Actor for Pong {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.send_to_players(Message("BEG".to_owned()));
        self.tick(ctx);
    }
}

impl Handler<Tick> for Pong {
    type Result = ();

    fn handle(&mut self, _: Tick, ctx: &mut Self::Context) {
		self.last_tick_time = Instant::now();
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
		println!("{}: {}", input.id, input.cmd);
        if self.players[0].id == input.id {
            self.players[0].last_input = input.cmd;
        } else {
            self.players[1].last_input = input.cmd;
        }
    }
}
