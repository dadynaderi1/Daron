pub struct Buffer {
    pub file: String,
    pub lines: Vec<String>,
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            file: String::from("[NO FILE]"),
            lines: vec!["Hello World!".to_string()],
        }
    }
}
