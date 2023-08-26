pub mod health_check;
pub mod auth_handlers;

pub mod prelude {
    pub use super::health_check;
    pub use super::auth_handlers;
}

pub mod all_auth_handlers {
    pub use super::auth_handlers::*;
}

pub mod all_util_hanlders {
    pub use super::health_check::{get_echo_time,get_health_check};
}