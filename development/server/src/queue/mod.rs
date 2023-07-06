use tokio::net::TcpListener;

mod bridge;
mod game;

use self::game::Game;
use self::game::Player;
use bridge::bridge;

pub(crate) async fn queue() {
    let listener = TcpListener::bind("127.0.0.1:4242").await.unwrap();

    let mut queue: Vec<Player> = Vec::<Player>::new();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let socket = tokio_tungstenite::accept_async(socket).await.unwrap();

        let (sender, receiver) = bridge(socket);

        queue.push(Player::new(sender, receiver));

        if queue.len() >= 2 {
            let p1 = queue.remove(0);
            let p2 = queue.remove(0);
            let game = Game::new([p1, p2]);

            tokio::spawn(async move {
                game.start().await;
            });
        }
    }
}
