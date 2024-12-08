#![allow(dead_code)]
#[derive(Debug)]
pub struct Highlight {
    pub id: i64,
    pub text: String,
    pub date_created: String,
    pub date_modified: String,
    pub style: Option<i32>,
    pub note: Option<String>,
}

impl Highlight {
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

