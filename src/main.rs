extern crate dotenv;
extern crate mongodb;

use actix_web::get;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use futures::stream::StreamExt;
use listenfd::ListenFd;
use mongodb::{
    bson::{doc, Bson},
    options::ClientOptions,
    Client,
};
use std::env;
use std::sync::Mutex;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("<h2>Welcome to Rust web server!</h2>")
}
#[get("/books")]
async fn index_book() -> impl Responder {
    ///let books_collection = collection();

    //Query the database for all pets which are cats.
    ////let mut cursor = books_collection.find(doc! {}, None).await.unwrap();

   //// while let Some(doc) = cursor.next().await {
   ///     println!("{}", doc.unwrap())
   //// }

    HttpResponse::Ok().body("Hello world again!")
}

// async fn index_book2() mongodb::Collection {
//     let books_collection = collection();

//     //Query the database for all pets which are cats.
//     let mut cursor = books_collection.find(doc! {}, None).await.unwrap();

//     while let Some(doc) = cursor.next().await {
//         println!("{}", doc.unwrap())
//     }

// }

// async fn index2(data: mongodb::Collection) -> impl Responder {
//     HttpResponse::Ok().body("Hello world again!")
// }

async fn collection() -> mongodb::Collection {
    dotenv::dotenv().expect("Failed to read .env file");
    let case_sensitive = env::var("MONGODB_URI").expect("MONGODB_URI not found");
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse(&case_sensitive.to_string())
        .await
        .unwrap();

    // Manually set an option.
    client_options.app_name = Some("XeonAPI".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options).unwrap();

    //let books_collection = db.collection("books");
    let books_collection = client.database("T").collection("books");

    //Query the database for all pets which are cats.
    let mut cursor = books_collection.find(doc! {}, None).await.unwrap();

    while let Some(doc) = cursor.next().await {
        println!("{}", doc.unwrap())
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();

    // dotenv::dotenv().expect("Failed to read .env file");
    // let case_sensitive = env::var("MONGODB_URI").expect("MONGODB_URI not found");
    // // Parse a connection string into an options struct.
    // let mut client_options = ClientOptions::parse(&case_sensitive.to_string())
    //     .await
    //     .unwrap();

    // // Manually set an option.
    // client_options.app_name = Some("XeonAPI".to_string());

    // // Get a handle to the deployment.
    // let client = Client::with_options(client_options).unwrap();

    // // List the names of the databases in that deployment.
    // for db_name in client.list_database_names(None, None).await.unwrap() {
    //     println!("{}", db_name);
    // }

    // //let books_collection = db.collection("books");
    // let books_collection = client.database("T").collection("books");

    //indexBook(books_collection);
    //#let fooo = index2(books_collection);

    // Query the database for all pets which are cats.
    //#let mut cursor = books_collection.find(doc! {}, None).await.unwrap();

    //# while let Some(doc) = cursor.next().await {
    //#     println!("{}", doc.unwrap())
    //# }

    let mut server = HttpServer::new(|| App::new().service(index).service(index_book));

    // let mut server = HttpServer::new(|| {
    //     App::new()
    //     .route("/again", web::get().to(index2))
    // });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8088")?
    };

    server.run().await
}
