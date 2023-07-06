use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc::{Receiver, Sender};

fn get_ms() -> u128 {
    let now = SystemTime::now();
    let duration = now
        .duration_since(UNIX_EPOCH)
        .expect("Failed to calculate duration");
    let milliseconds = duration.as_secs() as u128 * 1000 + u128::from(duration.subsec_millis());
    milliseconds
}

pub(crate) async fn start(mut receiver: Receiver<String>, sender: Sender<String>) {
    let min_time_per_tick_ms: u128 = 5;
    let length_per_ms: u128 = 1;
    let mut position: u128 = 500;

    let mut last_tick_time: u128 = get_ms();
    let mut status: char = 'n';

    let mut last_state = position;

    loop {
        if get_ms() <= last_tick_time {
            continue;
        }
        let time_since_last_tick = get_ms() - last_tick_time - 1;
        if time_since_last_tick < min_time_per_tick_ms {
            std::thread::sleep(Duration::from_millis(
                ((min_time_per_tick_ms / 10) + 1) as u64,
            ));
            continue;
        }

        last_tick_time += time_since_last_tick;

        // Get inputs of players
        match receiver.try_recv() {
            Ok(mut _status) => {
                println!("{}", _status);
                if let Some(c) = _status.chars().last() {
                    status = c;
                }
            }
            _ => {}
        }

        // Calculate game_state
        let length_traveled = length_per_ms * time_since_last_tick;
        if status == 'u' {
            position += length_per_ms * time_since_last_tick;
            if position > 10000 {
                position = 10000;
            }
        } else if status == 'd' {
            if position < length_traveled {
                position = 0;
            } else {
                position -= length_per_ms * time_since_last_tick;
            }
        }

        if last_state != position {
            last_state = position;
            println!("Position: {}", position);
            sender.send(position.to_string()).await.unwrap();
        }
    }
}
