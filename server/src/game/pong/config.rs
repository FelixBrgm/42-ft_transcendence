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
            min_time_per_tick_ms: Duration::from_millis(1000),
            paddle_speed: 6,
            ball_speed: 5,
            height: 900,
            width: 1600,
            paddle_length: 120,
        }
    }
}
