use crossterm;
mod handler;
use handler::terminal::{self, Terminal};
enum Mode {
    Normal,
    _Insert,
}
pub struct Editor {
    should_quit: bool,
    _mode: Mode,
    cursor_x: u16,
    cursor_y: u16,
}

impl Editor {
    pub fn default() -> Self {
        Editor {
            should_quit: false,
            _mode: Mode::Normal,
            cursor_x: 0,
            cursor_y: 0,
        }
    }
    fn initialize(&mut self) -> Result<(), std::io::Error> {
        Terminal::initialize()?;
        self.draw_rows();
        Terminal::move_cursor(self.cursor_x + 1, self.cursor_y)?;

        Ok(())
    }
    fn purge(&mut self) -> Result<(), std::io::Error> {
        Terminal::purge()?;
        Ok(())
    }
    fn draw_rows(&mut self) {
        let terminal_size = Terminal::get_size().unwrap().1;
        for i in 0..terminal_size {
            Terminal::move_cursor(0, i).unwrap();
            println!("~\r");
        }
    }
    pub fn run(&mut self) {
        if let Err(err) = self.repl() {
            panic!("{err:#?}")
        }
    }
    pub fn repl(&mut self) -> Result<(), std::io::Error> {
        self.initialize()?;
        loop {
            Terminal::move_cursor(self.cursor_x, self.cursor_y)?;
            match crossterm::event::read()? {
                crossterm::event::Event::Key(key_event) => match key_event.code {
                    crossterm::event::KeyCode::Char('q')
                        if key_event.modifiers == crossterm::event::KeyModifiers::CONTROL =>
                    {
                        self.should_quit = true;
                    }
                    crossterm::event::KeyCode::Tab => {
                        println!("Tab Pressed \r");
                        Terminal::clear_screen()?;
                        Terminal::move_cursor(self.cursor_x, self.cursor_y)?;
                    }
                    crossterm::event::KeyCode::Up => {
                        if self.cursor_y > 0 {
                            self.cursor_y -= 1;
                        }
                    }
                    crossterm::event::KeyCode::Down => {
                        if self.cursor_y == 0 || self.cursor_y <= Terminal::get_size().unwrap().1 {
                            self.cursor_y += 1;
                        }
                    }
                    crossterm::event::KeyCode::Right => {
                        if self.cursor_x == 0 || self.cursor_x <= Terminal::get_size().unwrap().0 {
                            self.cursor_x += 1;
                        }
                    }
                    crossterm::event::KeyCode::Left => {
                        if self.cursor_x > 0 {
                            self.cursor_x -= 1;
                        }
                    }
                    _ => (),
                },
                _ => (),
            }
            if self.should_quit {
                break;
            }
        }
        self.purge()?;
        Ok(())
    }
}
