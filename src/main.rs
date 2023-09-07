use actix_files::Files;
use actix_web::web;
use actix_web::{App, HttpServer};
use chrono::Utc;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use state::AppState;
use std::env;
use std::io;
use std::sync::{Arc, Mutex};

use crate::handlers::sse_handlers::Broadcaster;
mod handlers;
mod routes;
mod state;

#[actix_rt::main]
async fn main() -> io::Result<()> {

    //Load enviroment varibles.
    dotenv().ok();

    //Get DB connection URL.
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL is not present in .ENV file. Required.");
    let db_pool = PgPoolOptions::new()
        .max_connections(16)
        .connect(&database_url)
        .await
        .expect("Unable to connect to DB (connect)");

    //Initate State
    let app_state = web::Data::new(AppState {
        application_name: String::from("Actix Web Template DB:(Sqlx,Postgres,SSE,Websockets,Auth(JWT))"),
        health_check_count: Arc::new(Mutex::new(0)),
        last_check_time: Arc::new(Mutex::new(Utc::now())),
        pg_db: db_pool.clone(),
        sse_broadcaster: Broadcaster::create()
    });

    //Spin Up Web Server
    let server_ip = env::var("SERVER_IP").unwrap();
    let server_port = env::var("SERVER_PORT")
        .unwrap()
        .parse::<u16>()
        .unwrap();

    println!("💬 Server started @ {:}:{:}", server_ip, server_port);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(Files::new("/static", "."))
            .service(Files::new("/assets", "./static/assets"))
            .configure(routes::static_webserver_factory)
            .configure(routes::auth_routes_factory)
            .configure(routes::util_routes_factory)
            .configure(routes::sse_factory)
            .configure(routes::websocket_factory)
            .configure(routes::protected_routes_factory)
    })
    .bind((server_ip, server_port))?
    .run()
    .await

}
