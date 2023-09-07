use tokio::sync::mpsc::{self, Receiver, Sender};

mod bridge;
mod login;
mod runtime;
mod user;

use self::login::handle_socket_login;
use self::runtime::process_socket_data;
use self::user::User;

use self::runtime::Response;

pub struct RoomSocket {
    pub id: String,
    pub participant_uids: Vec<String>,
    pub buffer: Vec<Response>,
}

impl RoomSocket {
    pub fn new(id: String, participant_uids: Vec<String>) -> Self {
        RoomSocket {
            id,
            participant_uids,
            buffer: vec![],
        }
    }
}

pub async fn start_chat_server(
    room_update_sender: Sender<RoomSocket>,
    room_update_receiver: Receiver<RoomSocket>,
) {
    println!("Starting chat server...");
    let (login_successful_sender, login_successful_receiver) = mpsc::channel::<User>(2);

    let handle_1 = tokio::spawn(async move {
        handle_socket_login(login_successful_sender, room_update_sender).await;
    });

    let handle_2 = tokio::spawn(async move {
        process_socket_data(login_successful_receiver, room_update_receiver).await;
    });

    let _ = tokio::join!(handle_1, handle_2);
}
