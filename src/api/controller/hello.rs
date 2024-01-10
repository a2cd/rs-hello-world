use actix_web::{get, HttpResponse, Responder};

#[get("/hello")]
pub async fn say_hello() -> impl Responder {
    HttpResponse::Ok().body("hello")
}

#[get("/hello-world")]
pub async fn say_hello_world() -> impl Responder {
    HttpResponse::Ok().body("hello world")
}
