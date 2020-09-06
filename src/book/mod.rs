use actix_web::{web, HttpResponse, Responder};
use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    //bson::{doc, Bson},
    Client,
};
use rustc_serialize::hex::FromHex;
use std::sync::Mutex;

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/book/{id}").route(web::get().to(get_book)));
}

async fn get_book(data: web::Data<Mutex<Client>>, id: web::Path<String>) -> impl Responder {
    let book_one = data.lock().unwrap().database("T").collection("books");

    let bytes = id.from_hex().unwrap();
    let mut byte_array: [u8; 12] = [0; 12];
    for i in 0..12 {
        byte_array[i] = bytes[i];
    }
    // Query the database for all pets which are cats.
    let mut cursor = book_one
        .find(doc! {"_id": ObjectId::with_bytes(byte_array) }, None)
        .await
        .unwrap();

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
