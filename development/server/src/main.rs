use tokio::{io::AsyncReadExt, io::AsyncWriteExt, net::TcpListener};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:4242").await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        loop {
            let mut buf: [u8; 512] = [0; 512];

            let read_bytes: usize = socket.read(&mut buf).await.unwrap();
            if read_bytes != 0 {
                println!("{}", String::from_utf8_lossy(&buf));
                socket.write("Got: ".as_bytes()).await.unwrap();
                socket.write(&buf).await.unwrap();
            }
        }
        // socket.
    }
}
