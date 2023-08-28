use chrono::{DateTime, Utc};
use sqlx::postgres::PgPool;
use std::sync::{Arc, Mutex};

pub struct AppState {
    pub application_name: String,
    pub health_check_count: Arc<Mutex<i32>>,
    pub last_check_time: Arc<Mutex<DateTime<Utc>>>,
    pub pg_db: PgPool,
}
