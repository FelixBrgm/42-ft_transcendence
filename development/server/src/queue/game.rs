use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc::{Receiver, Sender};

pub struct Player {
    sender: Sender<String>,
    receiver: Receiver<String>,
    position: u16,
    last_input: char,
}
impl Player {
    pub fn new(sender: Sender<String>, receiver: Receiver<String>) -> Self {
        let position = 0;
        let last_input = 'n';
        Player {
            sender,
            receiver,
            position,
            last_input,
        }
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
}
impl GameConfig {
    fn new() -> Self {
        let min_time_per_tick_ms: u128 = 5;
        let length_per_ms: u128 = 10;
        let length: u16 = 10000;
        let width: u16 = 10000;

        GameConfig {
            min_time_per_tick_ms,
            length_per_ms,
            length,
            width,
        }
    }
}

pub struct Game {
    players: [Player; 2],
    ball: Ball,
    config: GameConfig,
    last_tick_time: u128,
}

impl Game {
    pub fn new(players: [Player; 2]) -> Self {
        let ball = Ball::new();
        let config = GameConfig::new();
        let last_tick_time = get_ms();
        Game {
            players,
            ball,
            config,
            last_tick_time,
        }
    }

    pub async fn start(mut self) {
        // let min_time_per_tick_ms: u128 = 5;
        // let length_per_ms: u128 = 1;
        // let mut position: u128 = 500;

        // let mut last_tick_time: u128 = get_ms();
        // let mut status: char = 'n';

        // let mut last_state = position;

        loop {
            self.tick().await;
        }
    }

    async fn tick(&mut self) {
        
        for i in 0..2 {
            self.players[i].write("testing".to_string()).await;
            self.players[i].write(i.to_string()).await;
        }
        std::thread::sleep(Duration::from_secs(1));
        // // Tick regulation
        // if get_ms() <= last_tick_time {
        //     continue;
        // }
        // let time_since_last_tick = get_ms() - last_tick_time - 1;
        // if time_since_last_tick < min_time_per_tick_ms {
        //     std::thread::sleep(Duration::from_millis(
        //         ((min_time_per_tick_ms / 10) + 1) as u64,
        //     ));
        //     continue;
        // }

        // last_tick_time += time_since_last_tick;

        // // Get inputs of players
        // match receiver.try_recv() {
        //     Ok(mut _status) => {
        //         println!("{}", _status);
        //         if let Some(c) = _status.chars().last() {
        //             status = c;
        //         }
        //     }
        //     _ => {}
        // }

        // // Calculate game_state
        // let length_traveled = length_per_ms * time_since_last_tick;
        // if status == 'u' {
        //     position += length_per_ms * time_since_last_tick;
        //     if position > 10000 {
        //         position = 10000;
        //     }
        // } else if status == 'd' {
        //     if position < length_traveled {
        //         position = 0;
        //     } else {
        //         position -= length_per_ms * time_since_last_tick;
        //     }
        // }

        // if last_state != position {
        //     last_state = position;
        //     println!("Position: {}", position);
        //     sender.send(position.to_string()).await.unwrap();
        // }
    }
}

fn get_ms() -> u128 {
    let now = SystemTime::now();
    let duration = now
        .duration_since(UNIX_EPOCH)
        .expect("Failed to calculate duration");
    let milliseconds = duration.as_secs() as u128 * 1000 + u128::from(duration.subsec_millis());
    milliseconds
}
