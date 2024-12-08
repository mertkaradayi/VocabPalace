mod models;
mod database;
mod ui;
mod config;

use rusqlite::Result;
use database::{connection, queries};
use ui::terminal;

fn main() -> Result<()> {
    let conn = connection::setup_database_connection()?;
    let books = queries::fetch_books(&conn)?;
    
    terminal::clear_screen();
    println!("📚 Available Books (sorted by last highlight date):\n");
    
    if let Some(selection) = terminal::show_book_selection(&books) {
        terminal::clear_screen();
        
        let selected_book = &books[selection];
        let highlights = queries::fetch_highlights(&conn, &selected_book.id)?;
        terminal::display_highlights(&selected_book.title, &highlights);
    }

    Ok(())
}