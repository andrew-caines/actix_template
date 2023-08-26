use crate::handlers::all_auth_handlers::{login, logout};
use crate::handlers::all_util_hanlders::get_health_check;
use crate::handlers::health_check::get_echo_time;

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
        .route("echo",get().to(get_echo_time))
    );
}
