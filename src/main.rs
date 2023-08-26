use actix_web::web;
use actix_web::{App, HttpServer};
use chrono::Utc;
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use state::AppState;
use std::env;
use std::io;
use std::sync::Mutex;
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
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Unable to connect to DB (connect)");

    //Spin up Webserver and initate state
    let app_state = web::Data::new(AppState {
        application_name: String::from("Actix Web Template DB:(Sqlx)"),
        health_check_count: Mutex::new(0),
        last_check_time: Mutex::new(Utc::now()),
        pg_db: db_pool,
    });
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(routes::auth_routes_factory)
            .configure(routes::util_routes_factory)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
