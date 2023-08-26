use crate::state::AppState;
use actix_web::{web, Responder};
use chrono::{DateTime, Utc};

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
    let diff =  now.time() - last_echo_time.time();
    let response = format!("{} time diff since last check: {} seconds", now, diff.num_seconds());
    *last_echo_time = now;
    response
}
