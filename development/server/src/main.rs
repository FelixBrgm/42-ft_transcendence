use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tungstenite::Message;

async fn handle_connection(stream: tokio::net::TcpStream) -> Result<(), Box<dyn std::error::Error>>
{
	let mut websocket = accept_async(stream).await.expect("Failed to accept");

	println!("Client connected");

	
	Ok(())
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Server started at {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream).await {
                eprintln!("Error: {}", e);
            }
        });
    }
}
