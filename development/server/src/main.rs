use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let listener = TcpListener::bind("0.0.0.0:4242").await?;
	
	loop {
		let (mut socket, _) = listener.accept().await?;

		tokio::spawn(async move {
			let mut buf = [0; 1024];

			// Read incoming data
			match socket.read(&mut buf).await {
				Ok(_) => {
					// Send "PONG" response
					if let Err(e) = socket.write_all(b"PONG\n").await {
						eprintln!("Failed to send response: {}", e);
					}
				}
				Err(e) => eprintln!("Failed to read from socket: {}", e),
			}
		});
	}
}
