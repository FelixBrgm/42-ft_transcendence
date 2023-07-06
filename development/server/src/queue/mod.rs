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

        let (sender, receiver, disconnect) = bridge(socket);

        queue.push(Player::new(sender, receiver, disconnect));

        remove_disconnected_sockets(&mut queue);

        if queue.len() >= 2 {
            let p1 = queue.remove(0);
            let p2 = queue.remove(0);
            let game = Game::new([p1, p2]);

            tokio::spawn(async move {
                println!("Game start!");
                game.start().await;
            });
        }
    }
}

fn remove_disconnected_sockets(queue: &mut Vec<Player>) {
    let mut i = 0;
    while i < queue.len() {
        if queue[i].is_disconnected() {
            queue.remove(i);
        } else {
            i += 1;
        }
    }
}
