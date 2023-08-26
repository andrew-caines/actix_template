use actix_web::Responder;

pub async fn login() -> impl Responder {
    String::from("Login-STUBBED")
}

pub async fn logout() -> impl Responder {
    String::from("Log out-STUBBED")
}