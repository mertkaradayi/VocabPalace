#![allow(dead_code)]
/// Represents a highlight in an iBook, including its text, metadata, and optional note
#[derive(Debug)]
pub struct Highlight {
    /// Unique identifier for the highlight
    pub id: i64,
    /// The highlighted text content
    pub text: String,
    /// Creation date in SQLite datetime format
    pub date_created: String,
    /// Last modification date in SQLite datetime format
    pub date_modified: String,
    /// Highlight color style (1-5) or None
    pub style: Option<i32>,
    /// Optional note attached to the highlight
    pub note: Option<String>,
}

impl Highlight {
    /// Converts the numeric style to a human-readable color name
    pub fn get_style_color(style: i32) -> &'static str {
        match style {
            1 => "Green",
            2 => "Blue",
            3 => "Yellow",
            4 => "Pink",
            5 => "Purple",
            _ => "Unknown",
        }
    }
}
