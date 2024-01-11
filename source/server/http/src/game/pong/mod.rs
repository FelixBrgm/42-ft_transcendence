mod ball;
mod config;
mod player;
// mod utils;

use actix::prelude::*;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};

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
pub struct UpdateScore {
    pub side: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct CountDown;

#[derive(Message)]
#[rtype(result = "()")]
pub struct GameStart;

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
	paused: bool,
}

impl Pong {
    pub fn new(players: [Player; 2]) -> Pong {
        Pong {
            players,
            score: [0; 2],
            ball: Ball::new(),
            config: GameConfig::new(),
            finished: false,
			paused: true,
        }
    }

    fn send_to_players(&self, msg: Message) {
        for player in &self.players {
            let _ = player.socket.do_send(msg.clone());
        }
    }

    fn send_pos(&mut self) {
        let msg = format!(
            "POS {:05} {:05} {:05} {:05}",
            self.players[0].position, self.players[1].position, self.ball.x, self.ball.y
        );
        self.send_to_players(Message(msg));
    }

    fn update(&mut self, ctx: &mut Context<Self>) {
        self.ball
            .update(10, &self.config, &mut self.players, &mut self.score, ctx);

        for player in self.players.iter_mut() {
            player.update(10, &self.config);
        }
    }

    fn tick(&mut self, ctx: &mut Context<Self>) {
        if self.finished || self.paused {
            return;
        }

        self.update(ctx);
        self.send_pos();

        ctx.run_later(TICK_INTERVAL, |_, ctx| {
            ctx.address().do_send(Tick);
        });
    }
}

impl Actor for Pong {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {

		self.players[0].socket.do_send(Message("FORMAT: {YOU} {OTHER} {BALL.x} {BALL.y}".to_owned()));
		self.players[1].socket.do_send(Message("FORMAT: {OTHER} {YOU} {BALL.x} {BALL.y}".to_owned()));

        self.ball.reset(&self.config);
		self.players[0].reset(&self.config);
		self.players[1].reset(&self.config);

        ctx.notify(CountDown);
    }
}

impl Handler<Tick> for Pong {
    type Result = ();

    fn handle(&mut self, _: Tick, ctx: &mut Self::Context) {
        self.tick(ctx);
    }
}


impl Handler<CountDown> for Pong {
	type Result = ();

	fn handle(&mut self, msg: CountDown, ctx: &mut Self::Context) {
		self.paused = true;

		let delay = 3;
		self.send_to_players(Message(format!("Starting game in {} Seconds", delay)));

		let ctx_addr = ctx.address();
        let slept = actix::clock::sleep(Duration::from_secs(delay)).into_actor(self);
        let fut = Box::pin(slept);
        let fut = fut.then(move |_r, _, _| {
            ctx_addr.do_send(GameStart);
            actix::fut::ready(())
        });

        ctx.spawn(fut);
	}
}

impl Handler<UpdateScore> for Pong {
    type Result = ();

    fn handle(&mut self, msg: UpdateScore, ctx: &mut Self::Context) {
		self.score[msg.side] += 1;
        self.send_to_players(Message(format!("SCORE {}:{}", self.score[0], self.score[1])));

		if self.score[msg.side] >= 3 {
			ctx.notify(GameOver);
		}
		else {
			ctx.notify(CountDown);
		}
    }
}

impl Handler<GameStart> for Pong {
    type Result = ();

    fn handle(&mut self, _: GameStart, ctx: &mut Self::Context) {
        self.paused = false;
        self.send_to_players(Message("BEG".to_owned()));
		self.tick(ctx);
    }
}

impl Handler<GameOver> for Pong {
    type Result = ();

	// TODO maybe call GameOver with and Enum that determines why or who won
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
