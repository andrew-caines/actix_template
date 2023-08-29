use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::time::{Duration, Instant};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

//Channels or Websocket scopes
pub struct EchoWs {
    hb: Instant,
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
