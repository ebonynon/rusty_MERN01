extern crate dotenv;
extern crate mongodb;

use actix_web::get;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
//use dotenv::dotenv;
use listenfd::ListenFd;
use mongodb::{options::ClientOptions, Client};
use std::env;

mod logs_handlers;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("<h1>Hello world!</h1>")
}
#[get("/again")]
async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}
#[get("/hello")]
async fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// #[cfg(feature = "tokio-runtime")]
// #[tokio::mongocall]
// async fn mongocall() -> mongodb::error::Result<()> {
//     dotenv::dotenv().expect("Failed to read .env file");
//     let case_sensitive = env::var("MONGODB_URI").expect("MONGODB_URI not found");

//     let mut client_options = ClientOptions::parse(&case_sensitive.to_string()).await?;
// }

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    // mongocall()
    dotenv::dotenv().expect("Failed to read .env file");
    let case_sensitive = env::var("MONGODB_URI").expect("MONGODB_URI not found");

    let mut client_options = ClientOptions::parse(&case_sensitive.to_string())
        .await
        .unwrap();

    // let mut client_options = ClientOptions::parse(&case_sensitive.to_string()).await?;

    // Manually set an option.
    client_options.app_name = Some("XeonAPI".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options).unwrap();

    // List the names of the databases in that deployment.
    for db_name in client.list_database_names(None, None).await.unwrap() {
        println!("{}", db_name);
    }

    let mut server = HttpServer::new(|| {
        App::new()
            .service(index)
            .service(index2)
            .service(index3)
            .service(web::scope("/api").configure(logs_handlers::scoped_config))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8088")?
    };

    server.run().await
}
