use std::collections::HashMap;


#[derive(Debug, Clone)]
pub struct ChatServer {
	sessions: HashMap<usize, Recipient<Message>>,
}

impl ChatServer {
    pub fn new() -> ChatServer {
		println!("chat server is being created.");
        ChatServer{
			sessions: HashMap::new(),
		}
    }
}

impl Actor for ChatServer {
	type Context = Context<Self>;
}

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Connect> for ChatServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> usize {
        println!("Someone joined");
		0
    }
}

impl Handler<Disconnect> for ChatServer {
	type Result = ();

	fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
		println!("Someone disconnected");
	}
}


impl Handler<ClientMessage> for ChatServer {
	type Result = ();

	fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
		println!("should send message");
	}
}