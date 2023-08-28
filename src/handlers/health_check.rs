use std::sync::{Mutex, Arc, MutexGuard};

use crate::state::AppState;
use actix_web::{web, HttpResponse, Responder};
use chrono::{DateTime, NaiveDateTime, Utc, Duration};
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

fn get_time_diff(start_time: &Arc<Mutex<DateTime<Utc>>>) -> Duration {
    //This function is broken out, so that you can visually see the drop of the Mutex lock, when this function ends.
    //The guard rails put on this, was created since stress testing cause the Mutex to Poison, this way if it does get poisoned
    //its immeditly corrected.

    let now: DateTime<Utc> = Utc::now();
    let mut last_echo_time: MutexGuard<DateTime<Utc>> = match start_time.lock()
    {
        Ok(guard) => guard,
        Err(poisoned) => {
            //Handle Mutex Poisoning
            let guard = poisoned.into_inner();
            println!("[get_time_diff] >> Recovered from mutex poisoning: {:?}", *guard);
            guard
        }
    };
    let diff = now.time() - last_echo_time.time();
    *last_echo_time = now;
    diff
}

pub async fn get_echo_time(app_state: web::Data<AppState>) -> impl Responder {
    //Calculate the difference between last visit to route. Then stash requestion in handler_log db.

    let now: DateTime<Utc> = Utc::now();
    let diff = get_time_diff(&app_state.last_check_time);
    let response = format!(
        "{} time diff since last check: {} seconds",
        now,
        diff.num_seconds()
    );
    let _ = sqlx::query!(
        "
        INSERT INTO
            handler_logs (handler, message)
        VALUES
        (
            'get_echo_time',
            'A Query was made to /util/echo'
        )"
    )
    .execute(&app_state.pg_db)
    .await;
    response
}

pub async fn get_handler_logs(app_state: web::Data<AppState>) -> impl Responder {
    //This route will get all entries in the handler_logs table and display them.
    let logs = sqlx::query_as::<_, HandlerLog>("SELECT * FROM handler_logs")
        .fetch_all(&app_state.pg_db)
        .await
        .unwrap();
    HttpResponse::Ok().json(logs)
}
