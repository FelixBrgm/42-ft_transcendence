mod ball;
mod config;
mod player;

use actix::prelude::*;
use std::time::Duration;

pub use self::ball::Ball;
pub use self::config::GameConfig;
pub use self::player::Player;
use crate::api::game::{GameMode, Stop};
pub use crate::game::{Message, UserId};

const TICK_INTERVAL: Duration = Duration::from_millis(5);

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

// send game result to he hosting server
#[derive(Message)]
#[rtype(result = "()")]
pub struct GameResult {
    players: [Player; 2],
    winner: UserId,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct GameFinished {
    pub players: [UserId; 2],
    pub winner: UserId,
}

#[derive(Debug, Clone)]
pub struct Pong {
    players: [Player; 2],
    score: [u8; 2],
    ball: Ball,
    config: GameConfig,
    finished: bool,
    paused: bool,
    mode: GameMode,
}

impl Pong {
    pub fn new(players: [Player; 2], mode: GameMode) -> Pong {
        Pong {
            players,
            score: [0; 2],
            ball: Ball::new(),
            config: GameConfig::new(),
            finished: false,
            paused: true,
            mode,
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
            .update(&self.config, &mut self.players, &mut self.score, ctx);

        for player in self.players.iter_mut() {
            player.update(&self.config);
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
        self.players[0].socket.do_send(Message(
            "FORMAT: {YOU} {OTHER} {BALL.x} {BALL.y}".to_owned(),
        ));
        self.players[1].socket.do_send(Message(
            "FORMAT: {OTHER} {YOU} {BALL.x} {BALL.y}".to_owned(),
        ));

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
            println!("HELO1");
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
        self.send_to_players(Message(format!("SCR {}:{}", self.score[0], self.score[1])));
        println!("{}", self.score[msg.side]);
        if self.score[msg.side] >= 3 {
            ctx.notify(GameOver);
        } else {
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

        // if let GameMode::OneVsOne(_) = &self.mode {
        //     for p in self.players.iter_mut() {
        //         p.addr.do_send(Stop { id: p.id });
        //     }
        // }
        // if let GameMode::Matchmaking(_) = &self.mode {
        // }
        if let GameMode::Tournament(addr) = &self.mode {
            let mut winner = self.players[1].id;
            if self.score[0] > self.score[1] {
                winner = self.players[0].id;
            }
            addr.do_send(GameFinished {
                players: [self.players[0].id, self.players[1].id],
                winner,
            })
        }
        for p in self.players.iter_mut() {
            p.addr.do_send(Stop { id: p.id });
        }
    }
}

impl Handler<PlayerInput> for Pong {
    type Result = ();

    fn handle(&mut self, input: PlayerInput, ctx: &mut Self::Context) {
        if self.players[0].id == input.id {
            self.players[0].last_input = input.cmd;
        } else {
            self.players[1].last_input = input.cmd;
        }
    }
}
