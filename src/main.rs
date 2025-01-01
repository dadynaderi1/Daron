mod editor;

fn main() {
    fn file_open(path: &String) -> Option<String> {
        let file_content: String =
            std::fs::read_to_string(path.to_string()).unwrap_or("no file".to_string());
        Some(file_content)
    }
    let args: Vec<String> = std::env::args().collect();
    if let Some(first_arg) = args.get(1) {
        let content = Some(file_open(first_arg));
        println!("{:?}", content);
    } else {
        println!("there is no file")
    }

    let mut editor = editor::Editor::default();
    editor.run();
}
