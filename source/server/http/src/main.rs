use tokio::sync::mpsc;

mod chat;
mod http;

use chat::RoomSocket;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let (room_update_sender, room_update_receiver) = mpsc::channel::<chat::RoomSocket>(100);
    
    {
        let room_update_sender = room_update_sender.clone();
        let _ = tokio::spawn(chat::start_chat_server(
            room_update_sender,
            room_update_receiver,
        ));
    }

    http::start_actix_server(room_update_sender).await;

    Ok(()) // only so no error hehe
}