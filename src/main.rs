use actix_cors::Cors;
use actix_web::get;
use actix_web::{http, web, App, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;
use mongodb::{options::ClientOptions, Client};
use std::env;
use std::sync::Mutex;

mod add_book;
mod book;
mod books;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("<h3>Welcome to Rust web server!</h3>")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();

    dotenv::dotenv().expect("Failed to read .env file");
    let case_sensitive = env::var("MONGODB_URI").expect("MONGODB_URI not found");
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse(&case_sensitive.to_string())
        .await
        .unwrap();

    // Manually set an option.
    client_options.app_name = Some("XeonAPI".to_string());

    let client = web::Data::new(Mutex::new(Client::with_options(client_options).unwrap()));

    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .wrap(
                Cors::new() // <- Construct CORS middleware builder
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .service(
                web::scope("/api")
                    .configure(books::scoped_config)
                    .configure(book::scoped_config)
                    .configure(add_book::scoped_config),
            )
            .service(index)
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8088")?
    };

    server.run().await
}
