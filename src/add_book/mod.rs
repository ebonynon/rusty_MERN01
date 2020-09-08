use actix_web::{web, HttpResponse, Responder};
//use futures::stream::StreamExt;
use chrono::prelude::*;
use mongodb::{
    //bson::doc,
    bson::{doc, Bson},
    Client,
};
use serde::Deserialize;
use std::sync::Mutex;

#[derive(Deserialize)]
pub struct NewBook {
    pub title: String,
    pub isbn: String,
    pub author: String,
    pub description: String,
    pub published_date: String, //Date ??,
    pub publisher: String,
    //pub updated_date: String      //Date ??,
}

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/book")
            .route(web::post().to(add_book))
            .route(web::get().to(index)),
    );
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("<h3>Welcome to <h2>Add Book</h2> @ Rust web server!</h3>")
}

async fn add_book(data: web::Data<Mutex<Client>>, new_book: web::Json<NewBook>) -> impl Responder {
    let books_collection = data.lock().unwrap().database("T").collection("books");

    match books_collection
        .insert_one(
            doc! {
            "title": &new_book.title,
            "isbn": &new_book.isbn,
            "author": &new_book.author,
            "description": &new_book.description,
            "published_date": &new_book.published_date,
            "publisher": &new_book.publisher,
            "updated_date": Bson::DateTime(Utc::now())},
            None,
        )
        .await
    {
        Ok(db_result) => {
            if let Some(new_id) = db_result.inserted_id.as_object_id() {
                println!("New document inserted with id {}", new_id);
            }
            return HttpResponse::Created().json(db_result.inserted_id);
        }
        Err(err) => {
            println!("Failed! {}", err);
            return HttpResponse::InternalServerError().finish();
        }
    }
}
