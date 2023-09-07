use crate::state::AppState;
use actix_web::{
    web::{Data, Json, ReqData},
    HttpResponse, Responder,
};
use actix_web_httpauth::extractors::basic::BasicAuth;
//use argonautica::{Hasher, Verifier};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sqlx::FromRow;

//
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

//Anything you want to 'load' onto a users JWT you add to this TokenClaims. Should that be permission levels or other user-level settings.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TokenClaims {
    pub id: i32,
    pub username: String,
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
    let hashed_password = gen_hash(user.password);
    match sqlx::query_as::<_, UserNoPassword>(
        "INSERT INTO users (username, password)
        VALUES ($1, $2)
        RETURNING id, username",
    )
    .bind(user.username)
    .bind(hashed_password)
    .fetch_one(&state.pg_db)
    .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error)),
    }
}

pub fn gen_hash(password: String) -> String {
    let argon2 = Argon2::default();
    let env_hash = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
    let hash_secret = env_hash.as_bytes();
    let hash_b64 = SaltString::encode_b64(&hash_secret).unwrap();
    let salt = SaltString::from_b64(&hash_b64.as_str()).unwrap();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    hash
}

pub fn get_salt() -> SaltString {
    let env_hash = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
    let hash_secret = env_hash.as_bytes();
    let hash_b64 = SaltString::encode_b64(&hash_secret).unwrap();
    let salt = SaltString::from_b64(&hash_b64.as_str()).unwrap();
    salt
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

    //When a TokenClaims is created, this is how you embed information into the token.
    //If you want to add information, like say values from the DB (example: post count), just make the matching
    //Change to the TokenClaims struct, and updated in the is_valid section below.

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
                    let argon2 = Argon2::default();
                    let salt = get_salt();
                    let user_password_hash = argon2.hash_password(pass.as_bytes(), &salt).unwrap();
                    let is_valid = Argon2::default()
                        .verify_password(pass.as_bytes(), &user_password_hash)
                        .is_ok();

                    if is_valid {
                        let claims = TokenClaims {
                            id: user.id,
                            username: user.username,
                        };
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
    let salt = SaltString::generate(&mut OsRng);
    println!("{salt}");
    String::from("Log out-STUBBED")
}

pub async fn protected_test(
    _: Data<AppState>,
    req_user: Option<ReqData<TokenClaims>>,
) -> impl Responder {
    //If use gets here with invalid JWT it will give them 401
    let r = req_user.unwrap();
    format!(
        "Hello {} [{}] you got into /protected/protected_test",
        r.username, r.id
    )
}
