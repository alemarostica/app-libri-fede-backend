use serde::{Serialize, Deserialize};
use surrealdb::sql::Thing;

// Book data model for the database
// Thing is the SurrealDB id type
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Book {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub title: String,
    pub author: String,
    pub year: u16,
    pub house: String,
    pub volume: Option<u16>,
    pub topic: String,
    pub location: String,
    pub read: bool,
}

// Data model for creating a new book, without the id
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewBook {
    pub title: String,
    pub author: String,
    pub year: u16,
    pub house: String,
    pub volume: Option<u16>,
    pub topic: String,
    pub location: String,
    pub read: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookDTO {
    pub id: String,
    pub title: String,
    pub author: String,
    pub year: u16,
    pub house: String,
    pub volume: Option<u16>,
    pub topic: String,
    pub location: String,
    pub read: bool,    
}

impl From<Book> for BookDTO {
    fn from(book: Book) -> Self {
        Self {
            // Vedere questa roba mi fa piangere
            id: book.id.expect("Book ID should exist").id.to_string(),
            title: book.title,
            author: book.author,
            year: book.year,
            house: book.house,
            volume: book.volume,
            topic: book.topic,
            location: book.location,
            read: book.read,
        }
    }
}
