#[derive(Debug)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: Option<String>,
    pub content_type: Option<String>,
}

impl Book {
    pub fn get_content_type_string(content_type: i32) -> String {
        match content_type {
            1 => "EPUB".to_string(),
            2 => "PDF".to_string(),
            _ => "Unknown".to_string(),
        }
    }
} 