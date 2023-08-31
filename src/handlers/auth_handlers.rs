use crate::state::AppState;
use actix_web::{
    web::{Data, Json, ReqData},
    HttpResponse, Responder,
};
use actix_web_httpauth::extractors::basic::BasicAuth;
use argonautica::{Hasher, Verifier};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TokenClaims {
    id: i32,
}

#[derive(Deserialize)]
pub struct CreateUserBody {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, FromRow)]
struct UserNoPassword {
    id: i32,
    username: String,
}

#[derive(Serialize, FromRow)]
pub struct AuthUser {
    pub id: i32,
    pub username: String,
    pub password: String,
}

pub async fn create_user(state: Data<AppState>, body: Json<CreateUserBody>) -> impl Responder {
    let user: CreateUserBody = body.into_inner();
    let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(user.password)
        .with_secret_key(hash_secret)
        .hash()
        .unwrap();
    match sqlx::query_as::<_, UserNoPassword>(
        "INSERT INTO users (username, password)
        VALUES ($1, $2)
        RETURNING id, username",
    )
    .bind(user.username)
    .bind(hash)
    .fetch_one(&state.pg_db)
    .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error)),
    }
}

pub async fn login(state: Data<AppState>, credentials: BasicAuth) -> impl Responder {
    let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(
        std::env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set!")
            .as_bytes(),
    )
    .unwrap();
    let username = credentials.user_id();
    let password = credentials.password();

    match password {
        None => HttpResponse::Unauthorized().json("Must provide username and password"),
        Some(pass) => {
            match sqlx::query_as::<_, AuthUser>(
                "SELECT id, username, password FROM users WHERE username = $1",
            )
            .bind(username.to_string())
            .fetch_one(&state.pg_db)
            .await
            {
                Ok(user) => {
                    let hash_secret =
                        std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
                    let mut verifier = Verifier::default();
                    let is_valid = verifier
                        .with_hash(user.password)
                        .with_password(pass)
                        .with_secret_key(hash_secret)
                        .verify()
                        .unwrap();

                    if is_valid {
                        let claims = TokenClaims { id: user.id };
                        let token_str = claims.sign_with_key(&jwt_secret).unwrap();
                        HttpResponse::Ok().json(token_str)
                    } else {
                        HttpResponse::Unauthorized().json("Incorrect username or password")
                    }
                }
                Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error)),
            }
        }
    }
}

pub async fn logout() -> impl Responder {
    String::from("Log out-STUBBED")
}

pub async fn protected_test(
    _: Data<AppState>,
    req_user: Option<ReqData<TokenClaims>>,
) -> impl Responder {
    println!("[protected_test] => req_user: {:?}", req_user.unwrap());
    String::from("You got into /protected/protected_test")
}
