use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::{
    net::TcpListener,
    sync::mpsc::{self, Receiver, Sender},
};

mod engine;
mod websocket;

// Utils

async fn game() {
    let listener = TcpListener::bind("127.0.0.1:4242").await.unwrap();

    

    let (client_sender, client_reciever) = mpsc::channel::<String>(1);
    let (server_sender, server_reciever) = mpsc::channel::<String>(1);

    let (socket, _) = listener.accept().await.unwrap();

    let socket = tokio_tungstenite::accept_async(socket).await.unwrap();
    println!("Accepted Client");

    let bridge_handle = tokio::spawn(async {
        websocket::bridge(socket, client_sender, server_reciever).await;
    });

    let runtime_handle = tokio::spawn(async {
        engine::start(client_reciever, server_sender).await;
    });

    let _ = bridge_handle.await;
    let _ = runtime_handle.await;
}

#[tokio::main]
async fn main() {
    game().await;
}
