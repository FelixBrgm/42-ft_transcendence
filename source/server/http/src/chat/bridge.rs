use futures_util::{SinkExt, StreamExt};
use tokio::{
    net::TcpStream,
    sync::mpsc::{self, Receiver, Sender},
};
use tokio_tungstenite::WebSocketStream;
use tungstenite::Message;

#[derive(PartialEq)]
enum Status {
    Connected,
    Disconnect,
}

pub struct Connection {
    pub sender: Sender<String>,
    pub receiver: Receiver<String>,
    disconnect: Receiver<()>,
    status: Status,
}

impl Connection {
    pub fn new(
        sender: Sender<String>,
        receiver: Receiver<String>,
        disconnect: Receiver<()>,
    ) -> Self {
        Connection {
            sender,
            receiver,
            disconnect,
            status: Status::Connected,
        }
    }

    pub fn is_disconnected(&mut self) -> bool {
        // Additional if for performance to not call try_recv again
        if self.status == Status::Disconnect {
            return true;
        }
        if let Ok(_) = self.disconnect.try_recv() {
            self.status = Status::Disconnect;
            return true;
        }
        return false;
    }
}

pub fn create_websocket_connection(socket: WebSocketStream<TcpStream>) -> Connection {
    let (mscp_to_socket_sender, mut mscp_to_socket_receiver) = mpsc::channel::<String>(100);
    let (socket_to_mscp_sender, socket_to_mscp_receiver) = mpsc::channel::<String>(100);
    let (disconnect_sender, disconnect_receiver) = mpsc::channel::<()>(1);

    let (mut write, mut read) = socket.split();

    tokio::spawn(async move {
        loop {
            match read.next().await {
                Some(msg) => {
                    if let Ok(msg) = msg {
                        if let Ok(msg) = msg.into_text() {
                            // When dropped this fails and with break the read variable is dropped and the read is closed
                            if let Err(msg) = socket_to_mscp_sender.send(msg).await {
                                let _ = disconnect_sender.send(()).await;
                                break;
                            }
                        };
                    }
                }
                None => {
                    let _ = disconnect_sender.send(()).await;
                    break;
                } // Socket_to_mpsc_sender is dropped here that causes the channel to return None at the receiver
            }
        }
    });
    tokio::spawn(async move {
        loop {
            match mscp_to_socket_receiver.recv().await {
                Some(msg) => {
                    let _ = write.send(Message::Text(msg + "\n")).await;
                    let _ = write.flush().await;
                }
                None => {
                    let _ = write.close().await;
                    break;
                }
            }
        }
    });

    return Connection::new(
        mscp_to_socket_sender,
        socket_to_mscp_receiver,
        disconnect_receiver,
    );
}
