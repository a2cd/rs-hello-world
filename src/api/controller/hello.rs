use actix_web::{get, HttpResponse, Responder};

#[get("/hello")]
pub async fn say_hello() -> impl Responder {
    HttpResponse::Ok().body("hello")
}
