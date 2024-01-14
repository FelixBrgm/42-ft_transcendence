use super::{GameConfig, Player, Pong, UpdateScore};
use actix::{AsyncContext, Context};
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
enum Dir {
    Pos,
    Neg,
}

impl Dir {
    pub fn reverse(&self) {
        match self {
            Dir::Pos => Dir::Neg,
            Dir::Neg => Dir::Pos,
        };
    }
}

#[derive(Debug, Clone)]
pub struct Ball {
    pub x: u16,
    pub y: u16,
    dir_x: Dir,
    dir_y: Dir,
}

impl Ball {
    pub fn new() -> Self {
        Ball {
            x: 0,
            y: 0,
            dir_x: Dir::Pos,
            dir_y: Dir::Pos,
        }
    }

    pub fn update(
        &mut self,
        config: &GameConfig,
        players: &mut [Player; 2],
        score: &mut [u8; 2],
        ctx: &mut Context<Pong>,
    ) {
        let distance: u16 = config.ball_speed;

        if self.dir_x == Dir::Pos {
            self.x += distance;
        } else {
            self.x -= distance;
        }

        // make this direction more random
        if self.dir_y == Dir::Pos {
            self.y += distance;
        } else {
            self.y -= distance;
        }

        // Check for collisions with paddles
        for player in players.iter() {
            if self.collides_with_paddle(player, config) {
                self.dir_x.reverse();
            }
        }

        // check for collisions with the top or bottom wall
        if self.y <= 0 || self.y >= config.height {
            self.dir_y.reverse();
        }

        // if the ball is out of bounds
        // if self.x < 0 || self.x > config.width {

        // 	let scoring_side = if self.x < 0 { 1 } else { 0 };
        // 	ctx.notify(UpdateScore { side: scoring_side });

        // 	self.reset(config);
        // 	players[0].reset(config);
        // 	players[1].reset(config);
        // }
    }

    fn collides_with_paddle(&self, player: &Player, config: &GameConfig) -> bool {
        self.y >= player.position && self.y <= player.position + config.paddle_length
    }

    pub fn reset(&mut self, config: &GameConfig) {
        let mut rng = rand::thread_rng();
        self.x = config.width / 2;
        self.y = config.height / 2;

        self.dir_x = Dir::Pos;
        self.dir_y = Dir::Pos;

        // // generate the ball pos in the middle third of the field
        // self.y = rng.gen_range(config.height / 3, (2 * config.height) / 3);;

        // self.dir_x = if rng.gen::<bool>() {1} else {-1};
        // self.dir_y = if rng.gen::<bool>() {1} else {-1};
    }
}
