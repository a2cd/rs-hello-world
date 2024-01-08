use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn print_index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to rust actix-web!")
}
