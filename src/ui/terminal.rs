use std::io::{stdout, Write};
use dialoguer::{theme::ColorfulTheme, Select};
use console::Term;
use crate::models::{Book, Highlight};

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    stdout().flush().unwrap();
}

pub fn show_book_selection(books: &[Book]) -> Option<usize> {
    if books.is_empty() {
        println!("No books with highlights found.");
        return None;
    }

    clear_screen();
    let term = Term::stdout();
    let height = term.size().1.saturating_sub(6);

    let mut options: Vec<String> = books
        .iter()
        .map(|book| {
            let author = &book.author;
            let content_type = book.content_type.as_deref().unwrap_or("Unknown Type");
            let title = if book.title.len() > 50 {
                format!("{}...", &book.title[..47])
            } else {
                book.title.clone()
            };
            format!("{} by {} • {}", title, author, content_type)
        })
        .collect();
    
    options.push("Exit".to_string());

    let theme = ColorfulTheme::default();
    let selection = Select::with_theme(&theme)
        .with_prompt("Select a book to view highlights")
        .default(0)
        .items(&options)
        .max_length(height as usize)
        .interact()
        .ok();

    match selection {
        Some(index) if index == options.len() - 1 => None,
        Some(index) => Some(index),
        None => None,
    }
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