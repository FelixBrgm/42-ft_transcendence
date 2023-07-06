mod ball;
mod game_config;
mod player;
mod utils;

pub use self::ball::Ball;
pub use self::game_config::GameConfig;
pub use self::player::Player;
use utils::get_ms;

use std::time::Duration;

pub struct Game {
    players: [Player; 2],
    ball: Ball,
    config: GameConfig,
    last_tick_time: u128,
    time_since_last_tick: u128,
    finished: bool,
}

impl Game {
    pub fn new(players: [Player; 2]) -> Self {
        let ball = Ball::new();
        let config = GameConfig::new();
        let last_tick_time = get_ms();
        let time_since_last_tick = 0;
        let finished = false;
        Game {
            players,
            ball,
            config,
            last_tick_time,
            time_since_last_tick,
            finished,
        }
    }

    pub async fn start(mut self) {
        while !self.finished {
            self.wait_till_next_tick().await;

            self.tick().await;

            for player in self.players.iter_mut() {
                if player.is_disconnected() {
                    self.finished = true;
                    return;
                }
            }
        }
        self.send_msg_to_players("END".to_owned()).await;
    }

    async fn tick(&mut self) {
        let length_traveled: u16 = (self.time_since_last_tick * self.config.length_per_ms) as u16;

        for player in self.players.iter_mut() {
            player.render(length_traveled, &self.config);
        }

        self.send_pos().await;
    }

    // Utils
    async fn send_pos(&mut self) {
        let msg = format!(
            "POS{:05}{:05}{:05}{:05}",
            self.players[0].position, self.players[1].position, self.ball.x, self.ball.y
        );
        self.send_msg_to_players(msg.to_string()).await;
    }

    async fn send_msg_to_players(&mut self, msg: String) {
        for player in self.players.iter_mut() {
            let _ = player.sender.send(msg.to_owned()).await;
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
}
