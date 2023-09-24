use super::{
    chat_commands::ChatCommand,
    messages::{ClientActorMessage, Connect, Disconnect, WsMessage},
};
use crate::{
    handlers::{
        chat_commands::{BroadcastMessage, WhisperMessage},
        lobby::Lobby,
    },
    state::AppState,
};
use actix::{
    fut, prelude::ContextFutureSpawner, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext,
    Handler, Running, StreamHandler, WrapFuture,
};
use actix_web::{
    web::{self, Data},
    Error, HttpRequest, HttpResponse,
};
use actix_web_actors::ws;
use std::{
    ops::Deref,
    time::{Duration, Instant},
};
use uuid::Uuid;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

//Channels or Websocket scopes
pub struct EchoWs {
    hb: Instant,
}

pub struct ItChannelWs {
    user_socket_id: Uuid,
    lobby_address: Addr<Lobby>,
    hb: Instant,
    it_channel_id: Uuid,
}

impl ItChannelWs {
    pub fn new(it_channel: Uuid, lobby: Addr<Lobby>) -> Self {
        Self {
            user_socket_id: Uuid::new_v4(),
            lobby_address: lobby,
            hb: Instant::now(),
            it_channel_id: it_channel,
        }
    }

    fn start_heartbeat(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket client for IT WS channel has disconnected.");
                // stop actor
                ctx.stop();
                // don't try to send a ping
                return;
            }
            ctx.ping(b"PING");
        });
    }
}

impl Actor for ItChannelWs {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.start_heartbeat(ctx);

        let addr = ctx.address();
        self.lobby_address
            .send(Connect {
                addr: addr.recipient(),
                lobby_id: self.it_channel_id,
                self_id: self.user_socket_id,
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_res) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.lobby_address.do_send(Disconnect {
            id: self.user_socket_id,
            room_id: self.it_channel_id,
        });
        Running::Stop
    }
}

impl EchoWs {
    pub fn new() -> Self {
        Self { hb: Instant::now() }
    }

    fn start_hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");
                // stop actor
                ctx.stop();
                // don't try to send a ping
                return;
            }
            ctx.ping(b"");
        });
    }
}

impl Actor for EchoWs {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.start_hb(ctx); //now start the HB for this connection.
    }
}

//StreamHandler for ItChannelWS
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ItChannelWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
            }
            Ok(ws::Message::Nop) => (),
            Ok(ws::Message::Text(text)) => {
                let s: &str = text.deref();
                let Ok(cmd) = serde_json::from_str(s) else {
                    println!("Some bullshit websocket message shape",);
                    return;
                };
                match cmd {
                    ChatCommand::Whisper { to_socket, message } => {
                        self.lobby_address.do_send(WhisperMessage {
                            channel_id: self.it_channel_id,
                            to_socket,
                            message,
                            user_socket_id: self.user_socket_id,
                        })
                    }
                    ChatCommand::Broadcast { message } => {
                        self.lobby_address.do_send(BroadcastMessage {
                            message: message,
                            user_socket_id: self.user_socket_id,
                            channel_id: self.it_channel_id,
                        })
                    }
                }
            }
            Err(e) => panic!("{:?}", e),
        }
    }
}

//Handlder for ws:Message(msg) for ItChannelWS
impl Handler<WsMessage> for ItChannelWs {
    type Result = ();
    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) -> Self::Result {
        //This is where you can massage or modify the incomming message. Maybe parse it, or precondition it before relaying to client.
        ctx.text(msg.0)
    }
}

//Handler for ws::Message (msg)
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for EchoWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now(); //Update latest ping time.
                ctx.pong(&msg)
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                //Whatever comes in here (text) is the Message body from client, whatever you write back to ctx is sent to client.
                //Right now, this just basically Echo's back what is sent with a bit of a change so you can see that its infact recieved and re-sent modified.
                let response = format!("You sent us: {}", text);
                ctx.text(response)
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                println!("WS Client Disconnected: {:#?}", &reason);
                ctx.close(reason);
                ctx.stop();
            }
            _ => (), //If i get a whack message, I close this connection.
        }
    }
}

//Routes
pub async fn ws_echo_handler(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let resp = ws::start(EchoWs::new(), &req, stream);
    //resp is now the Actor, now push that somewhere that can be tracked, so messages can be sent to it.
    println!("{:?}", resp);
    resp
}

pub async fn ws_it_handler(
    req: HttpRequest,
    stream: web::Payload,
    state: Data<AppState>,
) -> Result<HttpResponse, Error> {
    //Hard Coded IT Channel UUID:588b3104-c06c-4a1a-aad2-6f820309d1a0
    //TODO make it so fixed channels do not require UUID

    let ws = ItChannelWs::new(
        Uuid::parse_str("588b3104-c06c-4a1a-aad2-6f820309d1a0").unwrap(),
        state.lobby.clone(),
    );
    //println!("{:?}", &ws.it_channel_id);
    let resp = ws::start(ws, &req, stream)?;
    Ok(resp)
}
