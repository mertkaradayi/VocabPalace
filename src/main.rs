mod models;
mod database;
mod ui;
mod config;
mod file;

use rusqlite::Result;
use database::{connection, queries};
use ui::terminal;
use file::writer::HighlightWriter;

fn main() -> Result<()> {
    let conn = connection::setup_database_connection()?;
    let books = queries::fetch_books(&conn)?;
    
    terminal::clear_screen();
    println!("📚 Available Books (sorted by last highlight date):\n");
    
    if let Some(selection) = terminal::show_book_selection(&books) {
        terminal::clear_screen();
        
        let selected_book = &books[selection];
        let highlights = queries::fetch_highlights(&conn, &selected_book.id)?;
        
        // Display highlights in terminal
        terminal::display_highlights(&selected_book.title, &highlights);
        
        // Write highlights to file
        match HighlightWriter::new().and_then(|writer| writer.write_highlights(selected_book, &highlights)) {
            Ok(file_path) => println!("\n✅ Highlights saved to: {}", file_path.display()),
            Err(e) => eprintln!("\n❌ Failed to save highlights: {}", e),
        }
    }

    Ok(())
}