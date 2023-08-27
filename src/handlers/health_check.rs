use crate::state::AppState;
use actix_web::{web, HttpResponse, Responder};
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow, Debug)]
struct HandlerLog {
    log_id: i32,
    handler: String,
    message: Option<String>,
    created_at: NaiveDateTime,
}

pub async fn get_health_check(app_state: web::Data<AppState>) -> impl Responder {
    let mut counter = app_state.health_check_count.lock().unwrap();
    *counter += 1;
    let response = format!(
        "{} is up, this route has been checked {} times.",
        app_state.application_name, counter
    );
    response
}

pub async fn get_echo_time(app_state: web::Data<AppState>) -> impl Responder {
    //This just echos back the server time to console, used for load testing / uptime testing etc, it will return the time in local date time.
    //Copy out old echo time, display diff, update last echo time

    let now: DateTime<Utc> = Utc::now();
    let mut last_echo_time = app_state.last_check_time.lock().unwrap();
    let diff = now.time() - last_echo_time.time();
    let response = format!(
        "{} time diff since last check: {} seconds",
        now,
        diff.num_seconds()
    );
    *last_echo_time = now;
    //Now write entry to DB handler_logs (helps with stress testing too! :))
    //TODO UNDER ANY LOAD THIS DOESNT WORK, CLOSE CONNECTION MANUALLY OR ??!@
    let _ = sqlx::query!(
        "
        INSERT INTO
            handler_logs (handler, message)
        VALUES
        (
            'get_echo_time',
            'A query as put into get_echo_time'
        )"
    )
    .execute(&app_state.pg_db)
    .await;
    response
}

pub async fn get_handler_logs(app_state: web::Data<AppState>) -> impl Responder {
    //This route will get all entries in the handler_logs table and display them.
    //sqlx::query_as("SELECT * FROM handler_logs")
    let logs = sqlx::query_as::<_, HandlerLog>("SELECT * FROM handler_logs")
        .fetch_all(&app_state.pg_db)
        .await
        .unwrap();
    //println!("{:?}", logs);
    HttpResponse::Ok().json(logs)
}
