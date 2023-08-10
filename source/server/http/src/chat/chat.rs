use rand::Rng;
use std::sync::Arc;
use std::time::Duration;
use std::vec;
use tokio::net::TcpListener;
use tokio::sync::mpsc::{self, Receiver, Sender};

use super::bridge::{create_websocket_connection, Connection};
use super::user::User;
use super::login::handle_socket_login;
use super::runtime::{process_socket_data, RoomSocket};

pub async fn chat_start() {
    // TODO
    // LOGIN wiht user http
    // room updates if they change something
    let (room_update_sender, room_update_receiver) = mpsc::channel::<RoomSocket>(100);

    {
        let room_update_sender = room_update_sender.clone();
        tokio::spawn(start_chat_server(room_update_sender, room_update_receiver));
    }

    println!("ROOM ADDED");
    let _ = room_update_sender.try_send(RoomSocket {
        id: String::from("Test"),
        participant_uids: vec![],
        buffer: vec![],
    });

    tokio::time::sleep(Duration::from_secs(15000)).await;
    // room_udpate_sender usage in the http server
}

async fn start_chat_server(
    room_update_sender: Sender<RoomSocket>,
    room_update_receiver: Receiver<RoomSocket>,
) {
    let (login_successful_sender, login_successful_receiver) = mpsc::channel::<User>(2);

    let handle_1 = tokio::spawn(async move {
        handle_socket_login(login_successful_sender, room_update_sender).await;
    });

    let handle_2 = tokio::spawn(async move {
        process_socket_data(login_successful_receiver, room_update_receiver).await;
    });

    let _ = tokio::join!(handle_1, handle_2);
}
