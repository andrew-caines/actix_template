use actix_files::NamedFile;
use actix_web::Responder;

pub async fn index_handler() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}
