use core::time;
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
    mut socket: TcpStream,
    client_sender: Sender<char>,
    mut server_reciever: Receiver<String>,
) {
    let mut socket = Arc::new(Mutex::new(socket));
    let mut buf: [u8; 512] = [0; 512];
    {
        let socket = socket.clone();

        tokio::spawn(async move {
            loop {
                let mut socket = socket.lock().await;
                let read_result = socket.try_read(&mut buf);
                match read_result {
                    Ok(read_bytes) if read_bytes > 0 => {
                        for i in 0..read_bytes {
                            let c: char = char::from(buf[read_bytes - 1 - i]);
                            if c == 'u' || c == 'd' || c == 'n' {
                                client_sender.send(c).await.unwrap();
                                break;
                            }
                        }
                    }
                    _ => {} // Connection closed or error occurred
                }
                buf = [0; 512];
            }
        });
    }
    {
        tokio::spawn(async move {
            loop {
                let message = server_reciever.try_recv();
                match message {
                    Ok(mut c) => {
                        c.push('\n');
                        let mut socket = socket.lock().await;
                        let _ = socket.write(c.as_bytes()).await;
                        let _ = socket.flush().await;
                    }
                    Err(_) => {} // Channel closed
                }
            }
        });
    }
}

async fn runtime(mut client_reviever: Receiver<char>, server_sender: Sender<String>) {
    // let MIN_TIME_PER_TICK_MS: i32 = 500;
    // let LENGTH_PER_MS: i32 = 1;
    // let mut position: i32 = 500;

    // let mut last_tick_time = get_ms();
    // let mut status: char = 'n';

    loop {
        server_sender.send("TESTMESSAGE".to_string()).await.unwrap();
        println!("SENT MESSAGE");
        std::thread::sleep(Duration::from_secs(1));
        // let time_since_last_tick = (get_ms() - last_tick_time) as i32;

        // println!("tslt: {}", time_since_last_tick);

        // if time_since_last_tick < MIN_TIME_PER_TICK_MS {
        //     continue;
        // }
        // last_tick_time += time_since_last_tick as u128;

        // println!("POSITION: {} | {}", position, get_ms());

        // match client_reviever.try_recv() {
        //     Ok(_status) => {
        //         println!("GOT: {}", _status);
        //         status = _status;
        //     }
        //     _ => {}
        // }

        // if status == 'u' {
        //     position += LENGTH_PER_MS * time_since_last_tick;
        //     if position > 1000 {
        //         position = 1000;
        //     }
        // } else if status == 'd' {
        //     position -= LENGTH_PER_MS * time_since_last_tick;
        //     if position < 0 {
        //         position = 0;
        //     }
        // }

        // server_sender.send(position.to_string()).await.unwrap();
    }
}

async fn game() {
    let listener = TcpListener::bind("127.0.0.1:4242").await.unwrap();

    let (client_sender, client_reciever) = mpsc::channel::<char>(1);
    let (server_sender, server_reciever) = mpsc::channel::<String>(10);

    let (socket, _) = listener.accept().await.unwrap();

    tokio::spawn(async {
        bridge(socket, client_sender, server_reciever).await;
    });

    let runtime_handle = tokio::spawn(async {
        runtime(client_reciever, server_sender).await;
    });

    let _ = runtime_handle.await;
}

#[tokio::main]
async fn main() {
    game().await;
}
