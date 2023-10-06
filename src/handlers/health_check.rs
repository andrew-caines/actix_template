use crate::state::AppState;
use actix_web::{web, HttpResponse, Responder};
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Serialize, FromRow, Debug)]
struct HandlerLog {
    log_id: i32,
    handler: String,
    message: Option<String>,
    created_at: NaiveDateTime,
}
#[derive(Serialize, Debug)]
struct HandlerLogsPlusCount {
    count: i64,
    logs: Vec<HandlerLog>,
}
#[derive(Serialize, FromRow, Debug)]
struct HanderLogNewEntry {
    handler: String,
    message: String,
}

#[derive(Deserialize)]
pub struct LogRequestRequiredParams {
    offset: i32,
    limit: i32,
}

pub async fn get_health_check(app_state: web::Data<AppState>) -> impl Responder {
    let mut counter = app_state.health_check_count.lock().unwrap();
    *counter += 1;
    let response = format!(
        "{} is up, this route has been checked {} times.",
        app_state.application_name, counter
    );
    //Emit this acction to all connected users of /sse/general
    app_state
        .sse_broadcaster
        .broadcast("A user has checked the Health Route")
        .await;
    response
}

fn get_time_diff(start_time: &Arc<Mutex<DateTime<Utc>>>) -> Duration {
    //This function is broken out, so that you can visually see the drop of the Mutex lock, when this function ends.
    //The guard rails put on this, was created since stress testing cause the Mutex to Poison, this way if it does get poisoned
    //its immeditly corrected.

    let now: DateTime<Utc> = Utc::now();
    let mut last_echo_time: MutexGuard<DateTime<Utc>> = match start_time.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            //Handle Mutex Poisoning
            let guard = poisoned.into_inner();
            println!(
                "[get_time_diff] >> Recovered from mutex poisoning: {:?}",
                *guard
            );
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
    let data = HanderLogNewEntry {
        handler: String::from("get_echo_time"),
        message: String::from("A Query was made to /util/echo"),
    };
    let _ = sqlx::query_as!(
        HanderLogNewEntry,
        "
        INSERT INTO
            handler_logs (handler, message)
        VALUES
        (
            $1,
            $2
        )",
        data.handler,
        data.message
    )
    .execute(&app_state.pg_db)
    .await;
    //Emit this acction to all connected users of /sse/general
    app_state
        .sse_broadcaster
        .broadcast("A user has checked the Serve Echo Test Route")
        .await;
    response
}

pub async fn get_handler_logs(
    app_state: web::Data<AppState>,
    query: web::Query<LogRequestRequiredParams>,
) -> impl Responder {
    //This route will get all entries in the handler_logs table and display them.
    //path: OFFSET,LIMIT
    println!(
        "[get_handler_logs]> GET Params: limit:{:?}  offset:{:?}",
        query.limit, query.offset
    );
    let logs = sqlx::query_as::<_, HandlerLog>(
        "SELECT log_id,handler,message,created_at FROM handler_logs LIMIT $1 OFFSET $2",
    )
    .bind(query.limit)
    .bind(query.offset)
    .fetch_all(&app_state.pg_db)
    .await
    .unwrap();

    let count: (i64,) = sqlx::query_as("SELECT COUNT(log_id) FROM handler_logs")
        .fetch_one(&app_state.pg_db)
        .await
        .unwrap();

    //let count = logs.len() as i64;
    let result = HandlerLogsPlusCount {
        count: count.0,
        logs: logs,
    };
    HttpResponse::Ok().json(result)
}
