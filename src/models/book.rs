/// Represents a book in the iBooks library that contains highlights
#[derive(Debug)]
pub struct Book {
    /// Unique identifier for the book
    pub id: String,
    /// Book title
    pub title: String,
    /// Book author
    pub author: String,
    /// Content type description (e.g., "iBooks", "PDF")
    pub content_type: Option<String>,
}

impl Book {
    /// Converts the numeric content type to a human-readable format
    pub fn get_content_type_string(content_type: i32) -> String {
        match content_type {
            1 | 2 => "iBooks".to_string(),
            3 => "PDF".to_string(),
            _ => "Unknown".to_string(),
        }
    }
}
