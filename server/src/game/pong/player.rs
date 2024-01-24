use actix::Addr;

use super::GameConfig;
use crate::{api::game::GameSession, game::Socket};

#[derive(Debug, Clone)]
pub struct Player {
    pub id: usize,
    pub socket: Socket,
    pub addr: Addr<GameSession>,
    pub position: u16,
    pub last_input: char,
}

impl Player {
    pub fn new(id: usize, socket: Socket, addr: Addr<GameSession>) -> Player {
        Player {
            id,
            socket,
            position: 0,
            last_input: 'n',
            addr,
        }
    }

    pub fn update(&mut self, config: &GameConfig) {
        let length_traveled: u16 = config.paddle_speed;

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

    pub fn reset(&mut self, config: &GameConfig) {
        self.position = config.height / 2 + config.paddle_length / 2;
        self.last_input = 'n';
    }
}
