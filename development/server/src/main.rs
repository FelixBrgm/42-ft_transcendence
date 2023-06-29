use std::sync::{Arc, Mutex};
use tokio::{io::AsyncReadExt, io::AsyncWriteExt, net::TcpListener, net::TcpStream};

// type Chat = Arc<Mutex<[String]>>;
type Clients = Arc<Mutex<Vec<TcpStream>>>;

async fn process(mut socket: TcpStream, clients: Clients) {
    loop {
        let mut buf: [u8; 512] = [0; 512];

        let read_bytes: usize = socket.read(&mut buf).await.unwrap();
        if read_bytes != 0 {
            println!("{}", String::from_utf8_lossy(&buf));
            let mut clients = clients.lock().unwrap();
            for client in clients.iter_mut() {
                client.write("Message: ".as_bytes()).await.unwrap();
                client.write(&buf).await.unwrap();
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // let chat: Chat = Arc::new(Mutex::new([]));
    let clients: Clients = Arc::new(Mutex::new(Vec::new()));

    let listener = TcpListener::bind("127.0.0.1:4242").await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        // let chat = chat.clone();
        let clients = clients.clone();
        let mut value = clients.lock().unwrap();
        value.push(socket);

        tokio::spawn(async move {
            process(socket, clients).await;
        });
    }
}
