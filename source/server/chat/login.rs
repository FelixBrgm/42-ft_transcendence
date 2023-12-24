use rand::Rng;
use tokio::{net::TcpListener, sync::mpsc::Sender};

use crate::chat::bridge::{self, Connection};
use crate::chat::user::User;
use crate::chat::RoomSocket;

pub async fn handle_socket_login(
    login_successful_sender: Sender<User>,
    room_update_sender: Sender<RoomSocket>,
) {
    let listener = TcpListener::bind("127.0.0.1:2121").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let socket = tokio_tungstenite::accept_async(socket).await.unwrap();
        let connection: Connection = bridge::create_websocket_connection(socket);

        let login_successful_sender = login_successful_sender.clone();
        let room_update_sender = room_update_sender.clone();

        tokio::spawn(async move {
            handle_login(connection, login_successful_sender, room_update_sender).await;
        });
    }
}

async fn handle_login(
    connection: Connection,
    login_successful_sender: Sender<User>,
    room_update_sender: Sender<RoomSocket>,
) {
    // HTTP Request
    // handle login logic
    // also handle a timeout so that it doesn't go on forever

    let random_string: String = (0..12)
        .map(|_| rand::thread_rng().sample(rand::distributions::Alphanumeric) as char)
        .collect();
    let _ = login_successful_sender
        .send(User::new(random_string, connection))
        .await;

    // send all rooms that are connected to the user through the roomupdate_sender
}
