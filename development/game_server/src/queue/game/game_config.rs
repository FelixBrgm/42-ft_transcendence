pub struct GameConfig {
    pub min_time_per_tick_ms: u128,
    pub length_per_ms_paddle: u16,
    pub length_per_ms_ball: u16,
    pub length: u16,
    pub width: u16,
    pub paddle_length: u16,
}
impl GameConfig {
    pub fn new() -> Self {
        let min_time_per_tick_ms: u128 = 100;
        let length_per_ms_paddle: u16 = 10;
        let length_per_ms_ball: u16 = 10;
        let length: u16 = 10000;
        let width: u16 = 10000;
        let paddle_length: u16 = 2000;

        GameConfig {
            min_time_per_tick_ms,
            length_per_ms_paddle,
            length_per_ms_ball,
            length,
            width,
            paddle_length,
        }
    }
}
