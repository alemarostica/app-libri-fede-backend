mod structs;
mod models;
mod routes;
mod cors;

use rocket::{launch, routes};
use rocket::http::Status;
use surrealdb::{engine::local::{Db, RocksDb}, Surreal};
use std::sync::Arc;
use cors::*;

use crate::routes::book::*;
use crate::structs::*;

// A Request guard to get a reference tot he database
// The `State` parameter in Rocket's route handlers gives us access to this
async fn db_state() -> Result<Arc<Surreal<Db>>> {
    let db = Surreal::new::<RocksDb>("./books.db").await
        .map_err(|e| rocket::response::status::Custom(Status::InternalServerError, format!("Failed to initialize database: {}", e)))?;
    db.use_ns("books_namespace").use_db("books_db").await
        .map_err(|e| rocket::response::status::Custom(Status::InternalServerError, format!("Failed to use namespace/database: {}", e)))?;

    Ok(Arc::new(db))
}

#[launch]
async fn rocket() -> _ {
    // Initialize the SurrealDB client and use it as managed state
    let db = db_state().await.unwrap_or_else(|e| {
        eprintln!("Initialization failed: {}, {}", e.0, e.1);
        panic!("Database initialization failed. Check your configuration.");
    });

    rocket::build()
        .manage(AppState { db })
        .mount("/", routes![create_book, get_all_books, get_book_by_id, update_book, toggle_read, delete_book])
        .attach(CORS)
}


