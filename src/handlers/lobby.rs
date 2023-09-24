use super::chat_commands::{BroadcastMessage, WhisperMessage};
use super::messages::{Connect, Disconnect, WsMessage};
use actix::prelude::{Actor, Context, Handler, Recipient};
use serde::Serialize;
use serde_json::json;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Serialize)]
enum WsMessageTypes {
    NewUserConnected,
    UserDisconnected,
}

#[derive(Serialize)]
struct ConnectionResponse {
    socket_id: String,
}

type Socket = Recipient<WsMessage>;

pub struct Lobby {
    sessions: HashMap<Uuid, Socket>,     //self id to self
    rooms: HashMap<Uuid, HashSet<Uuid>>, //room id  to list of users id
}
impl Actor for Lobby {
    type Context = Context<Self>;
}

impl Default for Lobby {
    fn default() -> Lobby {
        Lobby {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
        }
    }
}

impl Lobby {
    fn send_message(&self, message: &str, id_to: &Uuid) {
        if let Some(socket_recipient) = self.sessions.get(id_to) {
            let _ = socket_recipient.do_send(WsMessage(message.to_owned()));
        } else {
            println!("attempting to send message but couldn't find user id.");
        }
    }
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        if self.sessions.remove(&msg.id).is_some() {
            self.rooms
                .get(&msg.room_id)
                .unwrap()
                .iter()
                .filter(|conn_id| *conn_id.to_owned() != msg.id)
                .for_each(|user_id| {
                    let response = json!({
                        "type":"WS_DISCONNECTED_USER",
                        "socket_id": &msg.id,
                        "success":true
                    });
                    self.send_message(&response.to_string(), user_id);
                    //self.send_message(&format!("{} disconnected.", &msg.id), user_id)
                });
            if let Some(lobby) = self.rooms.get_mut(&msg.room_id) {
                if lobby.len() > 1 {
                    lobby.remove(&msg.id);
                } else {
                    //only one in the lobby, remove it entirely
                    self.rooms.remove(&msg.room_id);
                }
            }
        }
    }
}

impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        // create a room if necessary, and then add the id to it
        self.rooms
            .entry(msg.lobby_id)
            .or_insert_with(HashSet::new)
            .insert(msg.self_id);

        // send to everyone in the room that new uuid just joined
        self.rooms
            .get(&msg.lobby_id)
            .unwrap()
            .iter()
            .filter(|conn_id| *conn_id.to_owned() != msg.self_id)
            .for_each(|conn_id| {
                let response = json!({
                    "type":"WS_CONNECTION_NEWUSER",
                    "socket_id": &conn_id,
                    "success":true
                });
                self.send_message(&response.to_string(), conn_id)
            });

        // store the address
        self.sessions.insert(msg.self_id, msg.addr);

        let response = json!({
            "type":"WS_CONNECTION_SUCCESS",
            "socket_id": &msg.self_id,
            "success":true
        });
        // send self your new uuid
        self.send_message(&response.to_string(), &msg.self_id);
    }
}

//Custom Struct Types for Direct Message types.
impl Handler<WhisperMessage> for Lobby {
    type Result = ();
    fn handle(&mut self, msg: WhisperMessage, _ctx: &mut Self::Context) -> Self::Result {
        let response = json!({
            "type":"WS_MESSAGE_WHISPER",
            "from_socket": msg.user_socket_id,
            "to_socket": &msg.to_socket,
            "message": msg.message,
        });
        self.send_message(&response.to_string(), &msg.to_socket);
    }
}

impl Handler<BroadcastMessage> for Lobby {
    type Result = ();
    fn handle(&mut self, msg: BroadcastMessage, _ctx: &mut Self::Context) -> Self::Result {
        let message = &msg.message;
        self.rooms
            .get(&msg.channel_id)
            .unwrap()
            .iter()
            .for_each(|client| {
                let response = json!({
                    "type":"WS_MESSAGE",
                    "from_socket": &msg.user_socket_id,
                    "to_socket":"ALL",
                    "message":message,
                });
                self.send_message(
                    &format!("{}", serde_json::to_string_pretty(&response).unwrap()),
                    client,
                )
            });
    }
}

