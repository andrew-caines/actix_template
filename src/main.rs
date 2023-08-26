use actix_web::web;
use actix_web::{App, HttpServer};
use chrono::Utc;
use state::AppState;
use std::sync::Mutex;
mod handlers;
mod routes;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                application_name: String::from("Actix Web Template DB:(Sqlx)"),
                health_check_count: Mutex::new(0),
                last_check_time: Mutex::new(Utc::now()),
            }))
            .configure(routes::auth_routes_factory)
            .configure(routes::util_routes_factory)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
