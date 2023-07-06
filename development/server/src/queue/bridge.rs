use futures_util::{SinkExt, StreamExt};
use tokio::{
    net::TcpStream,
    sync::mpsc::{self, Receiver, Sender},
};
use tokio_tungstenite::WebSocketStream;
use tungstenite::Message;

pub(crate) fn bridge(
    socket: WebSocketStream<TcpStream>,
) -> (Sender<String>, Receiver<String>, Receiver<()>) {
    let (mscp_to_socket_sender, mut mscp_to_socket_receiver) = mpsc::channel::<String>(1);
    let (socket_to_mscp_sender, socket_to_mscp_receiver) = mpsc::channel::<String>(1);
    let (disconnect_sender, mut disconnect_receiver) = mpsc::channel::<()>(1);

    let (mut write, mut read) = socket.split();

    tokio::spawn(async move {
        loop {
            match read.next().await {
                Some(msg) => {
                    if let Ok(msg) = msg {
                        if let Ok(msg) = msg.into_text() {
                            let _ = socket_to_mscp_sender.send(msg).await;
                        };
                    }
                }
                None => {
                    let _ = disconnect_sender.send(()).await;
                    return;
                } // Socket_to_mpsc_sender is dropped here that causes the channel to return None at the receiver
            }
        }
    });
    tokio::spawn(async move {
        loop {
            match mscp_to_socket_receiver.recv().await {
                Some(msg) => {
                    let _ = write.send(Message::Text(msg + "\n")).await;
                }
                None => {
                    let _ = write.close().await;
                    return;
                }
            }
        }
    });

    return (
        mscp_to_socket_sender,
        socket_to_mscp_receiver,
        disconnect_receiver,
    );
}
