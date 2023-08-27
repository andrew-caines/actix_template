pub mod auth_handlers;
pub mod health_check;

pub mod prelude {
    pub use super::auth_handlers;
    pub use super::health_check;
}

pub mod all_auth_handlers {
    pub use super::auth_handlers::*;
}

pub mod all_util_hanlders {
    pub use super::health_check::{get_echo_time, get_handler_logs, get_health_check};
}
