use actix::prelude::Message;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub enum ChatCommand {
    Whisper { to_socket: Uuid, message: String },
    Broadcast { message: String },
}

#[derive(Serialize, Message)]
#[rtype(result = "()")]
pub struct WhisperMessage {
    pub to_socket: Uuid,      //Target for Message (socket id)
    pub message: String,      //Message payload
    pub user_socket_id: Uuid, //Sender (socket id)
    pub channel_id: Uuid,     //Channel uuid, currently will be hard-coded to IT channel.
}

#[derive(Serialize, Message)]
#[rtype(result = "()")]
pub struct BroadcastMessage {
    pub message: String,      //Message payload
    pub user_socket_id: Uuid, //Sender (socket id)
    pub channel_id: Uuid,     //Channel uuid, currently will be hard-coded to IT channel.
}
