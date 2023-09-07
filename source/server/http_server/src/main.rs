use tokio::sync::mpsc;

pub mod chat;
mod http;

#[tokio::main]
async fn main() {
    let (room_update_sender, room_update_receiver) = mpsc::channel::<chat::RoomSocket>(100);
    {
        let room_update_sender = room_update_sender.clone();
        let _ = tokio::spawn(chat::start_chat_server(
            room_update_sender,
            room_update_receiver,
        ));
    }

    http::start_actix_server(room_update_sender).await;
}
