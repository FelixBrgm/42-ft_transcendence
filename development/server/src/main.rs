use std::sync::Arc;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    net::TcpStream,
    sync::{
        mpsc::{self, Receiver, Sender},
        Mutex,
    },
};

// // type Chat = Arc<Mutex<[String]>>;
// type Clients = Arc<Mutex<Vec<TcpStream>>>;

// async fn process(socket: TcpStream, clients: Clients, i: i32) {
//     loop {
//         let mut buf: [u8; 512] = [0; 512];

//         let mut clients: std::sync::MutexGuard<'_, Vec<TcpStream>> = clients.lock().unwrap();
//         let read_bytes: usize = clients[read(&mut buf).await.unwrap();
//         if read_bytes != 0 {
//             println!("{}", String::from_utf8_lossy(&buf));
//             for client in clients.iter_mut() {
//                 client.write("Message: ".as_bytes()).await.unwrap();
//                 client.write(&buf).await.unwrap();
//             }
//         }
//     }
// }

// #[tokio::main]
// async fn main() {
//     // let chat: Chat = Arc::new(Mutex::new([]));
//     let clients: Clients = Arc::new(Mutex::new(Vec::new()));

//     let listener = TcpListener::bind("127.0.0.1:4242").await.unwrap();

//     let i = 0;

//     loop {
// let (mut socket, _) = listener.accept().await.unwrap();

// // let chat = chat.clone();
// let clients = clients.clone();
// let mut value = clients.lock().unwrap();
// value.push(socket);

// tokio::spawn(async move {
//     process(socket, clients, i).await;
// });
//     }
// }

async fn bridge(
    mut socket: TcpStream,
    client_sender: Sender<char>,
    mut server_reciever: Receiver<String>,
) {
    let mut socket = Arc::new(Mutex::new(socket));
    let mut buf: [u8; 512] = [0; 512];
    {
        let socket = socket.clone();

        tokio::spawn(async move {
            loop {
                let mut socket = socket.lock().await;
                let read_result = socket.read(&mut buf).await;
                match read_result {
                    Ok(read_bytes) if read_bytes > 0 => {
                        for i in 0..read_bytes {
                            let c: char = char::from(buf[read_bytes - 1 - i]);
                            if c == 'u' || c == 'd' || c == 'n' {
                                client_sender.send(c).await.unwrap();
                                break;
                            }
                        }
                    }
                    _ => {} // Connection closed or error occurred
                }
                buf = [0; 512];
            }
        });
    }
    {
        tokio::spawn(async move {
            loop {
                let message = server_reciever.try_recv();
                match message {
                    Ok(c) => {
                        let mut socket = socket.lock().await;
                        let _ = socket.write(c.as_bytes()).await;
                    }
                    Err(_) => {} // Channel closed
                }
            }
        });
    }
}

async fn runtime(mut client_reviever: Receiver<char>, server_sender: Sender<String>) {
    server_sender.send("PONG:\n".to_string()).await.unwrap();
    loop {
        match client_reviever.try_recv() {
            Ok(message) => {
                println!("Received: {}", message);
            }
            Err(_) => {}
        }
    }
}

async fn game() {
    let listener = TcpListener::bind("127.0.0.1:4242").await.unwrap();

    let (client_sender, client_reciever) = mpsc::channel::<char>(1);
    let (server_sender, server_reciever) = mpsc::channel::<String>(10);

    let (socket, _) = listener.accept().await.unwrap();

    tokio::spawn(async {
        bridge(socket, client_sender, server_reciever).await;
    });

    let runtime_handle = tokio::spawn(async {
        runtime(client_reciever, server_sender).await;
    });

    let _ = runtime_handle.await;
}

#[tokio::main]
async fn main() {
    game().await;
}
// async fn main() {
//     // Create a channel
//     let (sender, mut receiver) = mpsc::channel::<char>(1);

//     // Spawn a task that sends messages
//     let send_task = task::spawn(async move {
//         for i in 0..5 {
//             let message = format!("Message {}", i);
//             sender.send(message).await.unwrap();
//             tokio::time::sleep(std::time::Duration::from_secs(1)).await;
//         }
//     });

//     // Spawn a task that receives messages
//     let receive_task = task::spawn(async move {
//         while let Some(message) = receiver.recv().await {
//             println!("Received: {}", message);
//         }
//     });

//     // Await both tasks
//     send_task.await.unwrap();
//     receive_task.await.unwrap();
// }
