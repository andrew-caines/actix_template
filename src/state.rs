use std::sync::Mutex;
use chrono::{DateTime, Utc};

pub struct AppState {
   pub application_name: String,
   pub health_check_count: Mutex<i32>,
   pub last_check_time: Mutex<DateTime<Utc>>,
}
