use super::{GameConfig, Player, Pong, UpdateScore};
use actix::{Context, AsyncContext};
use rand::Rng;

#[derive(Debug, Clone)]
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
    dir_x: i8,
    dir_y: i8,
}

impl Ball {
    pub fn new() -> Self {
        Ball {
            x: 0,
            y: 0,
            dir_x: 1,
            dir_y: 1,
        }
    }

	fn reverse_x(&mut self) {
		self.dir_x *= -1;
	}

	fn reverse_y(&mut self) {
		self.dir_y *= -1;
	}

	pub fn update(
		&mut self,
		time_since_last_tick: u16,
		config: &GameConfig,
		players: &mut [Player; 2],
		score: &mut [u8; 2],
		ctx: &mut Context<Pong>,
	) {
		let distance: u16 = time_since_last_tick * config.ball_speed;
	
		if self.dir_x == 1 {
			self.x += distance;
		} else {
			self.x -= distance;
		}
	
		// make this direction more random
		if self.dir_y == 1 {
			self.y += distance;
		} else {
			self.y -= distance;
		}
	
		// Check for collisions with paddles
		for player in players.iter() {
			if self.collides_with_paddle(player, config) {
				self.reverse_x();
			}
		}

		// check for collisions with the top or bottom wall
		if self.y <= 0 || self.y >= config.height {
			self.reverse_y();
		}
	
		// Check for scoring
		if self.x < 0 {
			ctx.notify(UpdateScore { side: 1 });
			self.reset(config);
			players[0].reset(config);
			players[1].reset(config);
		} else if self.x > config.width {
			ctx.notify(UpdateScore { side: 0 });
			self.reset(config);
			players[0].reset(config);
			players[1].reset(config);
		}
	}
	
	// Helper function to check collision with paddles
	fn collides_with_paddle(&self, player: &Player, config: &GameConfig) -> bool {
		self.y >= player.position && self.y <= player.position + config.paddle_length
	}

    // pub fn update(
    //     &mut self,
    //     time_since_last_tick: u16,
    //     config: &GameConfig,
    //     players: &[Player; 2],
    //     score: &mut [u8; 2],
	// 	ctx: &mut Context<Pong>,
    // ) {
    //     let distance: u16 = time_since_last_tick * config.ball_speed;

	// 	// if i go left and my ball pos is smaller than 
    //     if self.dir_x == -1 && self.x < distance {
    //         if self.player_has_scored(&players[0], config) {
    //             self.reset(config);
	// 			ctx.address().do_send(UpdateScore{side: 0});
    //         } else {
    //             self.reverse_x();
    //             self.x = distance - self.x;
    //         }
    //     } else if self.dir_x == 1 && self.x + distance > config.width {
    //         if self.player_has_scored(&players[1], config) {
    //             self.reset(config);
	// 			ctx.address().do_send(UpdateScore{side: 1})
    //         } else {
    //             self.reverse_x();
    //             self.x = config.width - ((self.x + distance) - config.width);
    //         }
    //     } else {
    //         if self.dir_x == 1 {
    //             self.x += distance;
    //         } else {
    //             self.x -= distance;
    //         }
    //     }

    //     if self.dir_y == -1 && self.y < distance {
    //         self.reverse_y();
    //         self.y = distance - self.y;
    //     } else if self.dir_y == 1 && (self.y + distance) >= config.width {
    //         self.reverse_y();
    //         self.y = config.width - ((self.y + distance) - config.width);
    //     } else {
    //         if self.dir_y == 1 {
    //             self.y += distance;
    //         } else {
    //             self.y -= distance;
    //         }
    //     }
    // }

    pub fn reset(&mut self, config: &GameConfig) {
        
        let mut rng = rand::thread_rng();
		self.x = config.width / 2;
		self.y = config.height / 2;

		// // generate the ball pos in the middle third of the field
        // self.y = rng.gen_range(config.height / 3, (2 * config.height) / 3);;

        // self.dir_x = if rng.gen::<bool>() {1} else {-1};
        // self.dir_y = if rng.gen::<bool>() {1} else {-1};
		self.dir_x = 1;
		self.dir_y = 1;

    }
	// shouldn't this be self.y since youre comparing it with the player pos 
    fn player_has_scored(&mut self, player: &Player, config: &GameConfig) -> bool {
        if player.position <= self.x && player.position + config.paddle_length >= self.x {
            return false;
        }
        return true;
    }
}
