pub struct Buffer {
    lines: Vec<String>,
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            lines: vec!["Hello World!".to_string()],
        }
    }
}
