use std::time::Duration;

#[derive(Debug, Clone)]
pub struct GameConfig {
    pub min_time_per_tick_ms: Duration,
    pub paddle_speed: u16,
    pub ball_speed: u16,
    pub height: u16,
    pub width: u16,
    pub paddle_length: u16,
}

impl GameConfig {
    pub fn new() -> GameConfig {
        GameConfig {
            min_time_per_tick_ms: Duration::from_millis(100),
            paddle_speed: 1000,
            ball_speed: 1000,
            height: 1600,
            width: 900,
            paddle_length: 2000,
        }
    }
}
