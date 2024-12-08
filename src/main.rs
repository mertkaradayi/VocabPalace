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
    println!("📚 iBooks Highlights Exporter");
    println!("============================\n");
    
    if books.is_empty() {
        println!("No books with highlights found in your iBooks library.");
        return Ok(());
    }
    
    println!("Found {} books with highlights\n", books.len());
    
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
        
        // Write highlights to files
        match HighlightWriter::new().and_then(|writer| writer.write_highlights(selected_book, &highlights)) {
            Ok((text_path, json_path)) => {
                println!("\n✅ Files saved successfully:");
                println!("   📝 Text file: {}", text_path.display());
                println!("   🔧 JSON file: {}", json_path.display());
            },
            Err(e) => eprintln!("\n❌ Failed to save highlights: {}", e),
        }
    } else {
        println!("\nGoodbye! 👋");
    }

    Ok(())
}