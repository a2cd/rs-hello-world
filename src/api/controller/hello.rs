use actix_web::{get, HttpResponse, Responder};
use chrono::{Local};

#[get("/hello")]
pub async fn say_hello() -> impl Responder {
    HttpResponse::Ok().body("hello")
}

#[get("/hello-world")]
pub async fn say_hello_world() -> impl Responder {
    HttpResponse::Ok().body("hello world")
}

#[get("/hello/time")]
pub async fn hello_time() -> impl Responder {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S");
    HttpResponse::Ok().body(now.to_string())
}
