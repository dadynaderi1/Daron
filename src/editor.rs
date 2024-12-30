pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Editor {}
    }
    pub fn run(&self) {
        crossterm::terminal::enable_raw_mode().unwrap();
        loop {
            match crossterm::event::read() {
                Ok(crossterm::event::Event::Key(event)) => {
                    println!("{event:?}\r");
                    if let crossterm::event::KeyCode::Char(c) = event.code {
                        if c == 'q' {
                            break;
                        }
                    }
                }
                Err(err) => {
                    println!("Error: {err}");
                }
                _ => (),
            }
        }
        crossterm::terminal::disable_raw_mode().unwrap();
    }
}
