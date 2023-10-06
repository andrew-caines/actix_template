/*
All these routes will be protected and will require User Admin to access.
 */

use actix_web::{
    web::{Data, ReqData},
    HttpResponse, Responder,
};

use serde::Serialize;
use sqlx::FromRow;

use super::auth_handlers::TokenClaims;
use crate::state::AppState;

#[derive(Serialize, FromRow, Debug)]
struct AllUsersResponse {
    users: Vec<User>,
    count: i32,
}

#[derive(Serialize, FromRow, Debug)]
struct User {
    id: i32,
    username: String,
}

//GET -> Returns all users, thier ID, and username, no password.
pub async fn get_all_users(
    state: Data<AppState>,
    req_user: Option<ReqData<TokenClaims>>,
) -> impl Responder {
    let r = req_user.unwrap();
    let response = format!("[{:?}] accessed /users/list ", r.username);
    let all_users = sqlx::query_as::<_, User>(
        "SELECT 
            id, 
            username
        FROM users;",
    )
    .fetch_all(&state.pg_db)
    .await
    .unwrap();
    //Count records, create AllUsersResponse and return it.

    //Emit this acction to all connected users of /sse/general??
    state.sse_broadcaster.broadcast(&response).await;
    let count = all_users.len() as i32;
    let result = AllUsersResponse {
        users: all_users,
        count: count,
    };
    HttpResponse::Ok().json(result)
}
