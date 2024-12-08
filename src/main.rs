// Module declarations
mod models;
mod database;
mod ui;
mod config;
mod file;

use rusqlite::Result;
use database::{connection, queries};
use ui::terminal;
use file::writer::HighlightWriter;

/// Main entry point for the iBooks Highlights Exporter
/// This application allows users to:
/// 1. View all books with highlights from their iBooks library
/// 2. Select a book to view its highlights
/// 3. Export the highlights to both text and JSON formats
fn main() -> Result<()> {
    // Initialize database connection
    let conn = connection::setup_database_connection()?;
    let books = queries::fetch_books(&conn)?;
    
    // Display application header
    terminal::clear_screen();
    println!("📚 iBooks Highlights Exporter");
    println!("============================\n");
    
    // Early return if no books found
    if books.is_empty() {
        println!("No books with highlights found in your iBooks library.");
        return Ok(());
    }
    
    println!("Found {} books with highlights\n", books.len());
    
    // Handle book selection and highlight export
    if let Some(selection) = terminal::show_book_selection(&books) {
        terminal::clear_screen();
        let selected_book = &books[selection];
        let highlights = queries::fetch_highlights(&conn, &selected_book.id)?;
        
        if highlights.is_empty() {
            println!("No highlights found for '{}'", selected_book.title);
            return Ok(());
        }
        
        // Display highlights in terminal
        terminal::display_highlights(&selected_book.title, &highlights);
        
        // Export highlights to files
        export_highlights(selected_book, &highlights);
    } else {
        println!("\nGoodbye! 👋");
    }

    Ok(())
}

/// Helper function to handle the export process and display results
fn export_highlights(book: &models::Book, highlights: &[models::Highlight]) {
    match HighlightWriter::new().and_then(|writer| writer.write_highlights(book, highlights)) {
        Ok((text_path, json_path)) => {
            println!("\n✅ Files saved successfully:");
            println!("   📝 Text file: {}", text_path.display());
            println!("   🔧 JSON file: {}", json_path.display());
        },
        Err(e) => eprintln!("\n❌ Failed to save highlights: {}", e),
    }
}