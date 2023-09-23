use crate::state::AppState;
use actix_rt::time::interval;
use actix_web::{web::Data, Responder};
use actix_web_lab::extract::Json;
use actix_web_lab::sse::{self, ChannelStream, Sse};
use futures_util::future;
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Deserialize,Debug)]
pub struct TestMessage {
    pub message: String,
}

#[derive(Debug, Clone, Default)]
struct BroadcasterInner {
    clients: Vec<sse::Sender>,
}

pub struct Broadcaster {
    inner: Mutex<BroadcasterInner>,
}

impl Broadcaster {
    //This is the main way to send messages to all connected clients.

    pub async fn broadcast(&self, msg: &str) {
        let clients = self.inner.lock().unwrap().clients.clone(); //get a list of clients
        let send_futures = clients
            .iter()
            .map(|client| client.send(sse::Data::new(msg))); //TODO make this a struct of types and use json sending of types.
                                                             //Now send to everybody, ignoring errors as every 10s we scan anyways.
        let _ = future::join_all(send_futures).await;
    }

    pub fn create() -> Arc<Self> {
        let this = Arc::new(Broadcaster {
            inner: Mutex::new(BroadcasterInner::default()),
        });
        Broadcaster::spawn_ping(Arc::clone(&this));
        this
    }

    pub async fn new_client(&self) -> Sse<ChannelStream> {
        println!("New Connection to SSE Endpoint.");
        let (tx, rx) = sse::channel(10);

        tx.send(sse::Data::new("connected")).await.unwrap();
        //println!("creating new clients success {:?}", tx);
        self.inner.lock().unwrap().clients.push(tx);
        rx
    }

    fn spawn_ping(this: Arc<Self>) {
        actix_web::rt::spawn(async move {
            let mut interval = interval(Duration::from_secs(20));
            loop {
                interval.tick().await;
                this.remove_stale_clients().await;
            }
        });
    }

    async fn remove_stale_clients(&self) {
        let clients = self.inner.lock().unwrap().clients.clone();
        // println!("Active Clients: {:?}", clients);
        let mut ok_clients = Vec::new();
        for client in clients {
            if client
                .send(sse::Event::Comment("ping".into()))
                .await
                .is_ok()
            {
                ok_clients.push(client.clone()); //They answered Ping, they are still fresh / available
            }
        }
        self.inner.lock().unwrap().clients = ok_clients; //Make the cleaned up list, the new list of clients.
    }
}

//Handlers here.
pub async fn new_sse_client(state: Data<AppState>) -> impl Responder {
    //When a user connects to this event stream /sse/general add them to the list of clients to broadcast too.
    state.sse_broadcaster.new_client().await
}

pub async fn test_broadcast(state: Data<AppState>, msg: Json<TestMessage>) -> impl Responder {
    println!("Got message: {:?}", &msg.message);
    state.sse_broadcaster.broadcast(&msg.message).await;
    String::from("Success")
}
