use tokio::{io::AsyncReadExt, io::AsyncWriteExt, net::TcpListener, net::TcpStream};

async fn process(mut socket: TcpStream) {
    loop {
        let mut buf: [u8; 512] = [0; 512];

        let read_bytes: usize = socket.read(&mut buf).await.unwrap();
        if read_bytes != 0 {
            println!("{}", String::from_utf8_lossy(&buf));
            socket.write("Got: ".as_bytes()).await.unwrap();
            socket.write(&buf).await.unwrap();
        }
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:4242").await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}
