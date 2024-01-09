
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
    pub fn new(
        id: usize, socket: Socket
    ) -> Player {
        Player {
			id,
			socket,
            position: 0,
            last_input: 'n',
        }
    }
}