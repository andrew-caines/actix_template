use crate::handlers::all_auth_handlers::{create_user, login, logout, protected_test, TokenClaims};
use crate::handlers::all_user_handlers::get_all_users;
use crate::handlers::all_util_handlers::get_health_check;
use crate::handlers::all_websocket_handlers::{ws_echo_handler, ws_it_handler};
use crate::handlers::health_check::{get_echo_time, get_handler_logs};
use crate::handlers::sse_handlers::{new_sse_client, test_broadcast};
use crate::handlers::static_handler::index_handler;
use actix_cors::Cors;
use actix_web::HttpMessage;
use actix_web::{
    dev::ServiceRequest,
    error::Error,
    web::{get, post, scope, ServiceConfig},
};
use actix_web_httpauth::extractors::bearer::{self, BearerAuth};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;

//Middlewar for JWT validiation
async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    //println!("[validator] Credentials: {:?}", credentials.token());
    let jwt_secret: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set!");
    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();
    let token_string = credentials.token();
    let claims: Result<TokenClaims, &str> = token_string
        .verify_with_key(&key)
        .map_err(|_| "Invalid token");

    match claims {
        Ok(value) => {
            req.extensions_mut().insert(value);
            Ok(req)
        }
        Err(_) => {
            let config = req
                .app_data::<bearer::Config>()
                .cloned()
                .unwrap_or_default()
                .scope("");
            println!("User supplied invalid JWT");
            Err((AuthenticationError::from(config).into(), req))
        }
    }
}

pub fn user_routes_factory(cfg: &mut ServiceConfig) {
    let bearer_middleware = HttpAuthentication::bearer(validator);
    let cors = Cors::permissive();
    cfg.service(
        scope("/users")
            .wrap(cors)
            .wrap(bearer_middleware)
            .route("list", get().to(get_all_users)),
    );
}

pub fn auth_routes_factory(cfg: &mut ServiceConfig) {
    //FOR DEV ONLY, REMOVE CORS STUFF FOR PRODUCTION OR UPDATE TO BE CORRECT
    let cors = Cors::permissive();
    cfg.service(
        scope("/v1/auth")
            .wrap(cors)
            .route("create", post().to(create_user))
            .route("login", post().to(login)) //This route is where you login and get JWT
            .route("logout", get().to(logout)),
    );
}

pub fn protected_routes_factory(cfg: &mut ServiceConfig) {
    let bearer_middleware = HttpAuthentication::bearer(validator);
    cfg.service(
        scope("/protected")
            .wrap(bearer_middleware)
            .route("test", post().to(protected_test)),
    );
}

pub fn util_routes_factory(cfg: &mut ServiceConfig) {
    //FOR DEV ONLY, REMOVE CORS STUFF FOR PRODUCTION OR UPDATE TO BE CORRECT
    let cors = Cors::permissive();
    cfg.service(
        scope("/util")
            .wrap(cors)
            .route("health", get().to(get_health_check))
            .route("echo", get().to(get_echo_time))
            .route("logs", get().to(get_handler_logs)),
    );
}

pub fn websocket_factory(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/ws")
            .route("/echo", get().to(ws_echo_handler))
            .route("/it", get().to(ws_it_handler)),
    );
}

pub fn static_webserver_factory(cfg: &mut ServiceConfig) {
    cfg.service(scope("/").route("", get().to(index_handler)));
}

pub fn sse_factory(cfg: &mut ServiceConfig) {
    let cors = Cors::permissive();
    cfg.service(
        scope("/sse")
            .wrap(cors)
            .route("general", get().to(new_sse_client))
            .route("test_message", post().to(test_broadcast)),
    );
}
