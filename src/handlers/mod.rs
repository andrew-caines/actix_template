pub mod auth_handlers;
pub mod health_check;
pub mod static_handler;
pub mod web_socket_handlers;

pub mod prelude {
    pub use super::auth_handlers;
    pub use super::health_check;
}

pub mod all_auth_handlers {
    pub use super::auth_handlers::*;
}

pub mod all_util_handlers {
    pub use super::health_check::{get_echo_time, get_handler_logs, get_health_check};
}

pub mod all_websocket_handlers {
    pub use super::web_socket_handlers::ws_echo_handler;
}
