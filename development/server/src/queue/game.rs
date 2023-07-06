use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc::{Receiver, Sender};

pub struct Player {
    sender: Sender<String>,
    receiver: Receiver<String>,
    disconnect: Receiver<()>,
    position: u16,
    last_input: char,
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

    pub async fn read(&mut self) -> Option<String> {
        self.receiver.recv().await
    }

    pub fn try_recv(&mut self) -> Option<String> {
        loop {
            match self.receiver.try_recv() {
                Ok(message) => return Some(message),
                Err(_) => break,
            }
        }
        None
    }

    pub async fn write(
        &mut self,
        message: String,
    ) -> Result<(), tokio::sync::mpsc::error::SendError<String>> {
        self.sender.send(message).await
    }
}

pub struct Ball {
    x: u16,
    y: u16,
}
impl Ball {
    fn new() -> Self {
        Ball { x: 0, y: 0 }
    }
}

struct GameConfig {
    min_time_per_tick_ms: u128,
    length_per_ms: u128,
    length: u16,
    width: u16,
    paddle_length: u16,
}
impl GameConfig {
    fn new() -> Self {
        let min_time_per_tick_ms: u128 = 50;
        let length_per_ms: u128 = 1;
        let ball_speed_div: u16 = 2;
        let length: u16 = 10000;
        let width: u16 = 10000;
        let paddle_length: u16 = 2000;

        GameConfig {
            min_time_per_tick_ms,
            length_per_ms,
            length,
            width,
            paddle_length,
        }
    }
}

pub struct Game {
    players: [Player; 2],
    ball: Ball,
    config: GameConfig,
    last_tick_time: u128,
    time_since_last_tick: u128,
}

impl Game {
    pub fn new(players: [Player; 2]) -> Self {
        let ball = Ball::new();
        let config = GameConfig::new();
        let last_tick_time = get_ms();
        let time_since_last_tick = 0;
        Game {
            players,
            ball,
            config,
            last_tick_time,
            time_since_last_tick,
        }
    }

    pub async fn start(mut self) {
        loop {
            self.wait_till_next_tick().await;
            if self.has_player_disconnected() {
                return;
            }
            self.tick().await;
        }
    }

    async fn tick(&mut self) {
        // Get last input
        for i in 0..2 {
            if let Ok(mut last_input) = self.players[i].receiver.try_recv() {
                println!("P{}: {}", i, last_input);
                if let Some(c) = last_input.chars().last() {
                    self.players[i].last_input = c;
                }
            }
        }

        self.update_player_position();

        self.send_pos().await;
    }
    // Pos
    fn update_player_position(&mut self) {
        let length_traveled: u16 = (self.time_since_last_tick * self.config.length_per_ms) as u16;

        for i in 0..2 {
            let player = &mut self.players[i];

            match player.last_input {
                'd' => {
                    if player.position > length_traveled {
                        player.position -= length_traveled;
                    } else {
                        player.position = 0;
                    }
                }
                'u' => {
                    player.position += length_traveled;
                    if player.position > self.config.length - self.config.paddle_length {
                        player.position = self.config.length - self.config.paddle_length;
                    }
                }
                _ => {}
            }
        }
    }

    // Utils
    async fn send_pos(&mut self) {
        let msg = format!(
            "POS{:05}{:05}{:05}{:05}",
            self.players[0].position, self.players[1].position, self.ball.x, self.ball.y
        );
        println!("{}", &msg);
        for i in 0..2 {
            let _ = self.players[i].write(msg.to_owned()).await;
        }
    }

    async fn wait_till_next_tick(&mut self) {
        loop {
            // This is so that it always takes 1ms steps minimum
            if get_ms() <= self.last_tick_time {
                std::thread::sleep(Duration::from_millis(1));
                continue;
            }

            self.time_since_last_tick = get_ms() - self.last_tick_time;

            if self.time_since_last_tick > self.config.min_time_per_tick_ms {
                self.last_tick_time = self.last_tick_time + self.time_since_last_tick;
                break;
            }

            std::thread::sleep(Duration::from_millis(
                ((self.config.min_time_per_tick_ms / 3) + 1) as u64,
            ));
        }
    }

    fn has_player_disconnected(&mut self) -> bool {
        for i in 0..2 {
            if self.players[i].is_disconnected() {
                println!("Game ended!");
                return true;
            }
        }
        return false;
    }
}

use std::time::Instant;

fn is_next_tick(current_tick_time: Instant, tick_interval: Duration) -> bool {
    let elapsed = current_tick_time.elapsed();
    elapsed >= tick_interval
}

fn get_ms() -> u128 {
    let now = SystemTime::now();
    let duration = now
        .duration_since(UNIX_EPOCH)
        .expect("Failed to calculate duration");
    let milliseconds = duration.as_secs() as u128 * 1000 + u128::from(duration.subsec_millis());
    milliseconds
}
