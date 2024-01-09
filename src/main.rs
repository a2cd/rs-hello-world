use std::io::Result;
use actix_web::{App, HttpServer};
use rs_hello_world::api::controller::*;

#[actix_web::main]
async fn main() -> Result<()> {
    let addrs = ("0.0.0.0", 8080);
    HttpServer::new(|| App::new().service(say_hello).service(print_index))
        .bind(addrs)?
        .run().await
}



