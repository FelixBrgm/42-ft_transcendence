use core::time;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    net::TcpStream,
    sync::{
        mpsc::{self, Receiver, Sender},
        Mutex,
    },
};
use tokio_tungstenite::WebSocketStream;
use tungstenite::protocol::{Role, WebSocketConfig};
use tungstenite::Message;

// Utils
fn get_ms() -> u128 {
    let now = SystemTime::now();
    let duration = now
        .duration_since(UNIX_EPOCH)
        .expect("Failed to calculate duration");
    let milliseconds = duration.as_secs() as u128 * 1000 + u128::from(duration.subsec_millis());
    milliseconds
}

async fn bridge(
    socket: WebSocketStream<TcpStream>,
    client_sender: Sender<char>,
    mut server_reciever: Receiver<String>,
) {
    let (mut write, mut read) = socket.split();
    {
        tokio::spawn(async move {
            loop {
                let res = read.next().await.unwrap();
                match res {
                    Ok(msg) => {
                        let mut msg = msg.into_text().unwrap();
                        println!("Recieved: {}", &msg);
                        let c = msg.pop().unwrap();
                        if c == 'u' || c == 'd' || c == 'n' {
                            client_sender.send(c).await.unwrap();
                        }
                    }
                    _ => {}
                };
            }
        });
    }
    {
        tokio::spawn(async move {
            loop {
                let message = server_reciever.recv().await;
                match message {
                    Some(msg) => {
                        println!("Sending: {}", &msg);
                        write.send(Message::Text(msg + "\n")).await.unwrap();
                    }
                    None => {} // Channel closed
                }
            }
        });
    }
}

async fn runtime(mut client_reviever: Receiver<char>, server_sender: Sender<String>) {
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
        match client_reviever.try_recv() {
            Ok(_status) => {
                status = _status;
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
            server_sender.send(position.to_string()).await.unwrap();
        }
    }
}

async fn game() {
    let listener = TcpListener::bind("127.0.0.1:4242").await.unwrap();

    let (client_sender, client_reciever) = mpsc::channel::<char>(1);
    let (server_sender, server_reciever) = mpsc::channel::<String>(1);

    let (socket, _) = listener.accept().await.unwrap();

    let socket = tokio_tungstenite::accept_async(socket).await.unwrap();
    println!("Accepted Client");

    let bridge_handle = tokio::spawn(async {
        bridge(socket, client_sender, server_reciever).await;
    });

    let runtime_handle = tokio::spawn(async {
        runtime(client_reciever, server_sender).await;
    });

    let _ = bridge_handle.await;
    let _ = runtime_handle.await;
}

#[tokio::main]
async fn main() {
    game().await;
}
