use actix_web::{get, HttpResponse};
use actix_web::http::header::ContentType;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello, bird!"
}

#[get("-1/seek")]
async fn seek() -> HttpResponse {
    HttpResponse::Found()
        .content_type(ContentType::plaintext())
        .append_header(("Location", "https://www.youtube.com/watch?v=9Gc4QTqslN4"))
        .finish()
}
