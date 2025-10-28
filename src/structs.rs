use surrealdb::{Surreal, engine::local::Db};
use std::sync::Arc;

// Define a Result alias for cleaner error handling
pub type Result<T> = std::result::Result<T, rocket::response::status::Custom<String>>;

// A state struct to hold the database connection
// This allows rocket to manage a single, persistent database connection across all requests
pub struct AppState {
    pub db: Arc<Surreal<Db>>,
}
