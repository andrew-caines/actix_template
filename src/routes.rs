use crate::handlers::all_auth_handlers::{login, logout};
use crate::handlers::all_util_handlers::get_health_check;
use crate::handlers::all_websocket_handlers::ws_echo_handler;
use crate::handlers::health_check::{get_echo_time, get_handler_logs};
use crate::handlers::static_handler::index_handler;

use actix_web::web::{get, scope, ServiceConfig};

pub fn auth_routes_factory(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/v1/auth")
            .route("login", get().to(login))
            .route("logout", get().to(logout)),
    );
}

pub fn util_routes_factory(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/util")
            .route("health", get().to(get_health_check))
            .route("echo", get().to(get_echo_time))
            .route("logs", get().to(get_handler_logs)),
    );
}

pub fn websocket_factory(cfg: &mut ServiceConfig) {
    cfg.service(scope("/ws").route("/echo", get().to(ws_echo_handler)));
}

pub fn static_webserver_factory(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/")
            .route("", get().to(index_handler))
    );
}
