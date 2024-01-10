use std::time::Duration;

#[derive(Debug, Clone)]
pub struct GameConfig {
    pub min_time_per_tick_ms: Duration,
    pub length_per_ms_paddle: u16,
    pub length_per_ms_ball: u16,
    pub length: u16,
    pub width: u16,
    pub paddle_length: u16,
}

impl GameConfig {
    pub fn new() -> GameConfig {
        GameConfig {
            min_time_per_tick_ms: Duration::from_millis(100),
            length_per_ms_paddle: 10,
            length_per_ms_ball: 10,
            length: 10000,
            width: 10000,
            paddle_length: 2000,
        }
    }
}
