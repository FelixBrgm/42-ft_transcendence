use futures_util::{SinkExt, StreamExt};
use tokio::{
    net::TcpStream,
    sync::mpsc::{Receiver, Sender},
};
use tokio_tungstenite::WebSocketStream;
use tungstenite::Message;

pub(crate) async fn bridge(
    socket: WebSocketStream<TcpStream>,
    sender: Sender<String>,
    mut receiver: Receiver<String>,
) {
    let (mut write, mut read) = socket.split();
    {
        tokio::spawn(async move {
            loop {
                if let Some(Ok(msg)) = read.next().await {
                    if let Ok(msg) = msg.into_text() {
                        let _ = sender.send(msg).await;
                    };
                }
            }
        });
    }
    {
        tokio::spawn(async move {
            loop {
                if let Some(msg) = receiver.recv().await {
                    let _ = write.send(Message::Text(msg + "\n")).await;
                }
            }
        });
    }
}
