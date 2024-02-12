
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::{Filter, Rejection};
use db::DB;
mod db;
mod errors;
mod handlers;

type Result<T> =  std::result::Result<T, errors::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    pub id: String,
    pub name: String,
    pub author: String,
    pub num_pages: usize,
    pub added_at: DateTime<Utc>,
    pub tags: Vec<String>,
}

#[tokio::main]
async fn main()->Result<()> {
    let db = DB::init().await?;
    let book = warp::path("books");

    let book_routes = book
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handlers::create_book_handler)
        .or(book
            .and(warp::put())
            .and(warp::path::param())
            .and(warp::body::json())
            .and(with_db(db.clone()))
            .and_then(handlers::edit_book_handler))
        .or(book
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(handlers::delete_book_handler))
        .or(book
            .and(warp::get())
            .and(with_db(db.clone()))
            .and_then(handlers::books_list_handler));
    let r = book_routes.recover(errors::handle_rejection);
    println!("Starting the server on port 8080");
    warp::serve(r).run(([0, 0, 0, 0], 8080)).await;

    Ok(())
}


fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}