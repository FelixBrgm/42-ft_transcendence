use super::{GameConfig, Player, Pong};

#[derive(Debug, Clone)]
pub struct Ball {
    pub x: u16,
    pub y: u16,
    direction_x: i8,
    direction_y: i8
}

impl Ball {
    pub fn new() -> Self {
        Ball { x: 0, y: 0, direction_x: 1, direction_y: 1 }
    }

    // pub fn update(&mut self, time_since_last_tick: u16, config: &GameConfig, players: &[Player; 2], score: & mut [u8; 2]) {
    //     let length_traveled: u16 = time_since_last_tick * config.length_per_ms_paddle;
        
    //     if self.direction_x == -1 && self.x < length_traveled {
    //         if self.player_has_scored(&players[0], config){
    //             self.reset(config);
    //             score[1] += 1;
    //         } else {
    //             self.direction_x = 1;
    //             self.x = length_traveled - self.x;
    //         }
    //     } else if self.direction_x == 1 && self.x + length_traveled > config.width {
    //         if self.player_has_scored(&players[1], config){
    //             self.reset(config);
    //             score[0] += 1;
    //         } else {
    //             self.direction_x = -1;
    //             self.x = config.width - ((self.x + length_traveled) - config.width);
    //         }
    //     } else {
    //         if self.direction_x == 1 {
    //             self.x += length_traveled;
    //         } else {
    //             self.x -= length_traveled;
    //         }
    //     }

    //     if self.direction_y == -1 && self.y < length_traveled {
    //         self.direction_y = 1;
    //         self.y = length_traveled - self.y;
    //     } else if self.direction_y == 1 && self.y + length_traveled > config.width {
    //         self.direction_y = -1;
    //         self.y = config.width - ((self.y + length_traveled) - config.width);
    //     } else {
    //         if self.direction_y == 1 {
    //             self.y += length_traveled;
    //         } else {
    //             self.y += length_traveled;
    //         }
    //     }
    // }

    // pub fn reset(&mut self, config: &GameConfig) {
    //     self.x = config.width / 2;
    //     self.y = config.length / 2;
    //     self.direction_x = -1;
    //     self.direction_y = -1;
    // }

    // fn player_has_scored(&mut self, player: &Player, config : &GameConfig) -> bool {
    //     if player.position < self.x && player.position + config.paddle_length > self.x {
    //         return true;
    //     }
    //     return false;
    // }
}
