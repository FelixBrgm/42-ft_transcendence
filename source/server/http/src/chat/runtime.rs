
use serde::Deserialize;
use tokio::sync::mpsc::Receiver;
use tokio::time::Duration;


use super::RoomSocket;
use super::user::User;

#[derive(Deserialize, Debug)]
pub struct Request {
    room_id: String,
    msg: String,
}

#[derive(serde::Serialize, Debug)]
pub struct Response {
    room_id: String,
    msg: String,
    user_id: String,
}

impl Response {
    fn clone(&self) -> Self {
        Response {
            room_id: String::from(self.room_id.to_string()),
            msg: String::from(self.msg.to_string()),
            user_id: String::from(self.user_id.to_string()),
        }
    }
}

pub async fn process_socket_data(
    mut login_successful_receiver: Receiver<User>,
    mut room_update_receiver: Receiver<RoomSocket>,
) {
    let mut users: Vec<User> = Vec::new();
    let mut rooms: Vec<RoomSocket> = Vec::new();

    loop {
        // remove disconnected users
        users.retain_mut(|user| !user.is_disconnected());

        // Check for newly connected users and add them
        while let Ok(user) = login_successful_receiver.try_recv() {
            users.retain(|u| u.uid != user.uid);
            users.push(user);
        }

        // Update based on room changes
        while let Ok(room) = room_update_receiver.try_recv() {
            rooms.retain(|r| r.id != room.id);
            rooms.push(room);
        }

        // try recv all messages
        for user in users.iter_mut() {
            while let Ok(msg) = user.connection.receiver.try_recv() {
                match serde_json::from_str::<Request>(&msg) {
                    Ok(req) => match rooms.iter_mut().find(|r| r.id == req.room_id) {
                        Some(room) => room.buffer.push(Response {
                            room_id: String::from(room.id.to_string()),
                            msg: String::from(req.msg.to_string()),
                            user_id: String::from(user.uid.to_string()),
                        }),
                        None => (),
                    },
                    Err(_err) => (),
                }
            }
        }

        // Send out buffers for each room
        for room in rooms.iter_mut() {
            for res in room.buffer.iter() {
                for uid in room.participant_uids.iter() {
                    if let Some(user) = users.iter_mut().find(|usr| usr.uid == uid.to_string()) {
                        let _ = user
                            .connection
                            .sender
                            .try_send(serde_json::to_string::<Response>(res).unwrap());
                        let res = res.clone();
                        tokio::spawn(async move {
                            send_msg_to_http(res).await;
                        });
                    }
                }
            }
            room.buffer.clear();
        }
        // send out all buffers
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

async fn send_msg_to_http(res: Response) {
    let _ = res;
    // HTTP request to the server to save the message/response in the database
}
