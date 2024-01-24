pub mod matchmake;

pub mod one_vs_one;
mod pong;
pub mod tournament;

use actix::prelude::*;

pub type Socket = Recipient<Message>;
pub type UserId = usize;

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub id: UserId,
    pub socket: Socket,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct TournamentConnect {
    pub tournament_id: UserId,
    pub uid: UserId,
    pub socket: Socket,
}
#[derive(Message)]
#[rtype(result = "()")]
pub struct Create {
    pub id: UserId,
    pub size: u8,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: UserId,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub id: UserId,
    pub msg: String,
}
