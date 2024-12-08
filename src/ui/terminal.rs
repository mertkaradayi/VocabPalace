use std::io::{stdout, Write};
use dialoguer::{theme::ColorfulTheme, Select};
use crate::models::{Book, Highlight};

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    stdout().flush().unwrap();
}

pub fn show_book_selection(books: &[Book]) -> Option<usize> {
    let options: Vec<String> = books
        .iter()
        .map(|book| {
            let author = book.author.as_deref().unwrap_or("Unknown Author");
            let content_type = book.content_type.as_deref().unwrap_or("Unknown Type");
            format!("{} by {} ({})", book.title, author, content_type)
        })
        .collect();

    Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a book to view highlights")
        .items(&options)
        .default(0)
        .interact()
        .ok()
}

pub fn display_highlights(book_title: &str, highlights: &[Highlight]) {
    println!("\nHighlights for '{}':\n", book_title);
    for highlight in highlights {
        let style = highlight.style
            .map(Highlight::get_style_color)
            .unwrap_or("No Style");
            
        println!("🔍 [{}] ({})", highlight.date_created, style);
        println!("   {}", highlight.text);
        
        if let Some(note) = &highlight.note {
            println!("   📝 Note: {}", note);
        }
        
        if highlight.date_modified != highlight.date_created {
            println!("   ✏️  Modified: {}", highlight.date_modified);
        }
        println!();
    }
} 