// Module declarations
mod cli;
mod config;
mod models;
mod repository;
mod utils;

use cli::app;
use config::connect_db;
use rusqlite::Result;

/// Main entry point for the iBooks Highlights Exporter
/// This application allows users to:
/// 1. View all books with highlights from their iBooks library
/// 2. Select a book to view its highlights
/// 3. Export the highlights to both text and JSON formats
fn main() -> Result<()> {
    // Initialize database connection
    let db_conn: rusqlite::Connection = connect_db()?;

    app(db_conn)?;

    Ok(())
}
