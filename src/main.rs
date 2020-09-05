use actix_web::{web, App, HttpResponse, HttpServer, Responder};

fn main() {
    println!("Hello, world!");

    async fn index() -> impl Responder {
        HttpResponse::Ok().body("Hello world!")
    }
    async fn index2() -> impl Responder {
        HttpResponse::Ok().body("Hello world again!")
    }
}
