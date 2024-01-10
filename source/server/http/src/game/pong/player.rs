use std::sync::{Arc, Mutex};

use super::GameConfig;
use crate::game::Socket;

#[derive(Debug, Clone)]
pub struct Player {
    pub id: usize,
    pub socket: Socket,
    pub position: u16,
    pub last_input: char,
}

impl Player {
    // TODO: position should be in the middle
    pub fn new(id: usize, socket: Socket) -> Player {
        Player {
            id,
            socket,
            position: 0,
            last_input: 'n',
        }
    }

    pub fn update(&mut self, time_since_last_tick: u16, config: &GameConfig) {
        let length_traveled: u16 = time_since_last_tick * config.paddle_speed;

        match self.last_input {
            'd' => {
                if self.position > length_traveled {
                    self.position -= length_traveled;
                } else {
                    self.position = 0;
                }
            }
            'u' => {
                self.position += length_traveled;
                if self.position > config.height - config.paddle_length {
                    self.position = config.height - config.paddle_length;
                }
            }
            _ => {}
        }
    }
}
