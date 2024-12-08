use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use crate::models::{Book, Highlight};

pub struct HighlightWriter {
    output_dir: PathBuf,
}

impl HighlightWriter {
    pub fn new() -> std::io::Result<Self> {
        // Create a directory in the user's home directory
        let home = dirs::home_dir().expect("Could not find home directory");
        let output_dir = home.join("ibooks_highlights");
        fs::create_dir_all(&output_dir)?;
        
        Ok(Self { output_dir })
    }

    pub fn write_highlights(&self, book: &Book, highlights: &[Highlight]) -> std::io::Result<PathBuf> {
        // Create a safe filename from the book title
        let safe_title = book.title.replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_");
        let filename = format!("{}.txt", safe_title);
        let file_path = self.output_dir.join(filename);
        
        let mut file = File::create(&file_path)?;

        // Write book information
        writeln!(file, "Book: {}", book.title)?;
        if let Some(author) = &book.author {
            writeln!(file, "Author: {}", author)?;
        }
        if let Some(content_type) = &book.content_type {
            writeln!(file, "Format: {}", content_type)?;
        }
        writeln!(file)?;
        writeln!(file, "Highlights:")?;
        writeln!(file, "===========")?;
        writeln!(file)?;

        // Write highlights
        for highlight in highlights {
            let style = highlight.style
                .map(Highlight::get_style_color)
                .unwrap_or("No Style");
                
            writeln!(file, "[{}] ({})", highlight.date_created, style)?;
            writeln!(file, "{}", highlight.text)?;
            
            if let Some(note) = &highlight.note {
                writeln!(file, "Note: {}", note)?;
            }
            
            if highlight.date_modified != highlight.date_created {
                writeln!(file, "Modified: {}", highlight.date_modified)?;
            }
            writeln!(file)?;
        }

        Ok(file_path)
    }
} 