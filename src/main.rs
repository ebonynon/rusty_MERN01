use actix_web::get;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use futures::stream::StreamExt;
use listenfd::ListenFd;
use mongodb::{
    bson::doc,
    //bson::{doc, Bson},
    options::ClientOptions,
    Client,
};
use std::env;
use std::sync::Mutex;

mod books;

fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/logs").route(web::get().to(get_logs)));
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("<h3>Welcome to Rust web server!</h3>")
}

async fn get_logs(data: web::Data<Mutex<Client>>) -> impl Responder {
    let logs_collection = data.lock().unwrap().database("T").collection("books");

    // Query the database for all pets which are cats.
    let mut cursor = logs_collection.find(doc! {}, None).await.unwrap();

    let mut results = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                results.push(document);
            }
            _ => {
                return HttpResponse::InternalServerError().finish();
            }
        }
    }
    HttpResponse::Ok().json(results)
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
            .service(
                web::scope("/api")
                    .configure(scoped_config)
                    .configure(books::scoped_config),
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
