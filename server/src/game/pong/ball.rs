use super::{GameConfig, Player, Pong, UpdateScore};
use actix::{AsyncContext, Context};
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
enum Dir {
    Pos,
    Neg,
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

        // Movement
        match self.dir_x {
            Dir::Pos => {
                if self.x + distance > config.width {
                    self.dir_x = Dir::Neg;
                    self.x = config.width - (distance - (config.width - self.x));
                    if !self.collides_with_paddle(&players[1], config) {
                        ctx.notify(UpdateScore { side: 0 });
                        self.reset(config);
                    }
                } else {
                    self.x += distance;
                }
            }
            Dir::Neg => {
                if distance > self.x {
                    self.dir_x = Dir::Pos;
                    self.x = distance - self.x;
                    if !self.collides_with_paddle(&players[0], config) {
                        ctx.notify(UpdateScore { side: 1 });
                        self.reset(config);
                    }
                } else {
                    self.x -= distance;
                }
            }
        }

        match self.dir_y {
            Dir::Pos => {
                if self.y + distance > config.height {
                    self.dir_y = Dir::Neg;
                    self.y = config.height - (distance - (config.height - self.y));
                } else {
                    self.y += distance;
                }
            }
            Dir::Neg => {
                if distance > self.y {
                    self.dir_y = Dir::Pos;
                    self.y = distance - self.y;
                } else {
                    self.y -= distance;
                }
            }
        }
    }

    fn collides_with_paddle(&self, player: &Player, config: &GameConfig) -> bool {
        self.y >= player.position && self.y <= player.position + config.paddle_length
    }

    pub fn reset(&mut self, config: &GameConfig) {
        let mut rng = rand::thread_rng();
        self.x = config.width / 2;

        let min_y = config.height / 4;
        self.y = rng.gen_range(min_y..min_y * 3);

        self.dir_x = if rng.gen_bool(0.5) {
            Dir::Pos
        } else {
            Dir::Neg
        };
        self.dir_y = if rng.gen_bool(0.5) {
            Dir::Pos
        } else {
            Dir::Neg
        };
    }
}
