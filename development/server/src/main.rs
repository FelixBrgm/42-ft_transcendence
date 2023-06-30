use tokio::{
    io::AsyncReadExt,
    net::TcpListener,
    net::TcpStream,
    sync::mpsc::{self, Receiver, Sender},
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

async fn bridge(mut socket: TcpStream, sender: Sender<char>) {
    loop {
        tokio::select! {
            let mut buf: [u8; 512] = [0; 512];
    
            let read_bytes: usize = socket.read(&mut buf).await.unwrap();
            if read_bytes != 0 {
                for i in 0..read_bytes {
                    let c: char = char::from(buf[read_bytes -1 -i]);
                    if c == 'u' || c == 'd' || c == 'n' {
                        sender.send(c).await.unwrap();
                        break;
                    }
                }
            }
            message = receiver.recv() => {
                match message {
                    Some(c) => {
                        let _ = socket.write(&[c as u8]).await;
                    }
                    None => break, // Channel closed
                }
            }
        }
    }
}

async fn runtime(mut reciever: Receiver<char>) {
    loop {
        match reciever.try_recv() {
            Ok(message) => println!("Received: {}", message),
            Err(_) => {}
        }
    }
}

async fn game() {
    let listener = TcpListener::bind("127.0.0.1:4242").await.unwrap();

    let (sender, mut reciever) = mpsc::channel::<char>(1);

    let (mut socket, _) = listener.accept().await.unwrap();

    tokio::spawn(async {
        bridge(socket, sender).await;
    });

    let runtime_handle = tokio::spawn(async {
        runtime(reciever).await;
    });

    runtime_handle.await;
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
