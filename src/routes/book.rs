use rocket::{get, post, put, patch,  delete, State};
use rocket::serde::json::Json;
use rocket::http::Status;

use crate::{AppState, Result};
use crate::models::book::*;

//--- CRUD routes ---

// Route to create a new book
#[post("/books", format = "json", data = "<new_book>")]
pub async fn create_book(db: &State<AppState>, new_book: Json<NewBook>) -> Result<Json<BookDTO>> {
    let db = &db.db;
    let created: Option<Book> = db.create("books").content(new_book.into_inner()).await
        .map_err(|e| rocket::response::status::Custom(Status::InternalServerError, format!("Failed to create book: {}", e)))?;

    created.map(BookDTO::from).map(Json).ok_or_else(|| rocket::response::status::Custom(Status::InternalServerError, "Failed to return created book".into()))
}

// Route to get all books
#[get("/books")]
pub async fn get_all_books(db: &State<AppState>) -> Result<Json<Vec<BookDTO>>> {
    let books: Vec<Book> = db.db.select("books").await
        .map_err(|e| rocket::response::status::Custom(Status::InternalServerError, format!("Failed to fetch books: {}", e)))?;
    let book_dtos: Vec<BookDTO> = books.into_iter().map(BookDTO::from).collect();
    Ok(Json(book_dtos))
}

// Route to get a single book by its id
#[get("/books/<id>")]
pub async fn get_book_by_id(db: &State<AppState>, id: &str) -> Result<Json<BookDTO>> {
    let book: Option<Book> = db.db.select(("books", id)).await
        .map_err(|e| rocket::response::status::Custom(Status::InternalServerError, format!("Failed to fetch book: {}", e)))?;
    book.map(BookDTO::from).map(Json).ok_or_else(|| rocket::response::status::Custom(Status::NotFound, "Book not found".into()))
}

// Route to update a book by its id
#[put("/books/<id>", format = "json", data = "<updated_book>")]
pub async fn update_book(db: &State<AppState>, id: &str, updated_book: Json<NewBook>) -> Result<Json<BookDTO>> {
    let updated: Option<Book> = db.db.update(("books", id)).content(updated_book.into_inner()).await
        .map_err(|e| rocket::response::status::Custom(Status::InternalServerError, format!("Failed to update book: {}", e)))?;
    updated.map(BookDTO::from).map(Json).ok_or_else(|| rocket::response::status::Custom(Status::NotFound, "Book not found".into()))
}

// Route to toggle read/unread status
#[patch("/books/<id>")]
// TODO: Darà problemi di sincronizzazione con la UI? Il me futuro se ne preoccuperà
pub async fn toggle_read(db: &State<AppState>, id: &str) -> Result<Json<BookDTO>> {
    let book: Option<Book> = db.db.select(("books", id)).await
        .map_err(|e| rocket::response::status::Custom(Status::InternalServerError, format!("Failed to fetch book: {}", e)))?;
    let mut book_to_update = book.ok_or_else(|| rocket::response::status::Custom(Status::NotFound, "Book not found".into()))?;
    book_to_update.read = !book_to_update.read;
    let updated: Option<Book> = db.db.update(("books", id)).content(book_to_update).await
        .map_err(|e| rocket::response::status::Custom(Status::InternalServerError, format!("Failed to update book: {}", e)))?;
    updated.map(BookDTO::from).map(Json).ok_or_else(|| rocket::response::status::Custom(Status::NotFound, "Book not found".into()))
}

// Route to delete a book by its id
#[delete("/books/<id>")]
pub async fn delete_book(db: &State<AppState>, id: &str) -> Result<Json<BookDTO>> {
    let deleted: Option<Book> = db.db.delete(("books", id)).await
        .map_err(|e| rocket::response::status::Custom(Status::InternalServerError, format!("Failed to delete book: {}", e)))?;
    deleted.map(BookDTO::from).map(Json).ok_or_else(|| rocket::response::status::Custom(Status::NotFound, "Book not found".into()))
}
