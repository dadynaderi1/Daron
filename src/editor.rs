mod handler;
use std::io::Write;
// Enviromental variables!
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
// End of enviromental variables
use handler::terminal::Terminal;
enum Mode {
    Normal,
    _Insert,
}
pub struct Editor {
    should_quit: bool,
    _mode: Mode,
    cursor_x: u16,
    cursor_y: u16,
    cols: u16,
    rows: u16,
}

impl Editor {
    pub fn default() -> Self {
        let size = Terminal::get_size().unwrap();
        Editor {
            should_quit: false,
            _mode: Mode::Normal,
            cursor_x: 0,
            cursor_y: 0,
            cols: size.0,
            rows: size.1,
        }
    }
    fn initialize(&mut self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;
        Terminal::initialize()?;
        self.draw_rows()?;
        Terminal::move_cursor(self.cursor_x + 1, self.cursor_y)?;
        Terminal::show_cursor()?;
        Ok(())
    }
    fn purge(&mut self) -> Result<(), std::io::Error> {
        Terminal::purge()?;
        Ok(())
    }
    fn draw_rows(&mut self) -> Result<(), std::io::Error> {
        for i in 0..self.rows {
            Terminal::move_cursor(0, i)?;
            if i == self.rows / 2 {
                self.splash_screen()?;
            } else {
                Terminal::print("~")?;
            }
        }
        Ok(())
    }
    fn splash_screen(&mut self) -> Result<(), std::io::Error> {
        //Terminal::show_cursor().unwrap();
        //Terminal::move_cursor(self.cols / 2 - 5, self.rows / 2).unwrap();
        let mut name = format!("{NAME} editor -- version: {VERSION}");
        let cols = self.cols as usize;
        let padding = (cols - name.len()) / 2;
        let spaces = " ".repeat(padding - 1);
        name = format!("~{spaces}{name}");
        name.truncate(cols);
        Terminal::print(name)?;
        Ok(())
        //Terminal::hide_cursor().unwrap();
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
                        Terminal::clear_screen()?;
                        Terminal::move_cursor(self.cursor_x, self.cursor_y)?;
                    }
                    crossterm::event::KeyCode::Up => {
                        if self.cursor_y > 0 {
                            self.cursor_y -= 1;
                        }
                    }
                    crossterm::event::KeyCode::Down => {
                        if self.cursor_y == 0 || self.cursor_y <= self.rows {
                            self.cursor_y += 1;
                        }
                    }
                    crossterm::event::KeyCode::Right => {
                        if self.cursor_x == 0 || self.cursor_x <= self.cols {
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
                crossterm::event::Event::FocusGained => todo!(),
                crossterm::event::Event::FocusLost => todo!(),
                crossterm::event::Event::Mouse(mouse_event) => todo!("{:?}", mouse_event),
                crossterm::event::Event::Paste(_) => todo!(),
                crossterm::event::Event::Resize(x, y) => {
                    self.cols = x;
                    self.rows = y;
                }
            }
            if self.should_quit {
                break;
            }
            std::io::stdout().flush()?;
        }
        self.purge()?;
        Ok(())
    }
}
