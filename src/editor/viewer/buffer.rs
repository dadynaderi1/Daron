pub struct Buffer {
    pub file: String,
    pub lines: Vec<String>,
}
fn handle_file(file: String) -> Vec<String> {
    let file = vec![file];
    file
}
pub fn open_file(path: String) -> Vec<String> {
    let file = std::fs::read_to_string(path).unwrap();
    let res = handle_file(file);
    res
}
impl Default for Buffer {
    fn default() -> Self {
        Self {
            lines: Vec::new(),
            file: "[No Name]".to_string(),
        }
    }
}
impl Buffer {
    pub fn new(arg: String) -> Self {
        let file = open_file(arg);
        Self {
            file: String::from("test"),
            lines: file,
        }
    }
}
