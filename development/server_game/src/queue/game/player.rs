use tokio::sync::mpsc::{Receiver, Sender};

use super::GameConfig;

pub struct Player {
    pub sender: Sender<String>,
    receiver: Receiver<String>,
    disconnect: Receiver<()>,
    pub position: u16,
    pub last_input: char,
}
impl Player {
    pub fn new(
        sender: Sender<String>,
        receiver: Receiver<String>,
        disconnect: Receiver<()>,
    ) -> Self {
        let position = 0;
        let last_input = 'n';
        Player {
            sender,
            receiver,
            disconnect,
            position,
            last_input,
        }
    }

    pub fn is_disconnected(&mut self) -> bool {
        if let Ok(_) = self.disconnect.try_recv() {
            return true;
        }
        return false;
    }

    pub fn render(&mut self, length_traveled: u16, config: &GameConfig) {
        // Get last_input
        if let Ok(last_input) = self.receiver.try_recv() {
            if let Some(c) = last_input.chars().last() {
                self.last_input = c;
            }
        }

        // Render new position
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
                if self.position > config.length - config.paddle_length {
                    self.position = config.length - config.paddle_length;
                }
            }
            _ => {}
        }
    }
}
