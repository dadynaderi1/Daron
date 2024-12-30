mod handler;
mod viewer;
use handler::terminal::Terminal;
use std::io::Write;
use viewer::Viewer;
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
    view: Viewer,
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
            view: viewer::Viewer,
        }
    }
    pub fn initialize(&mut self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;
        Terminal::initialize()?;
        self.view.renderer(self.rows, self.cols)?;
        Terminal::move_cursor(self.cursor_x + 1, self.cursor_y)?;
        Terminal::show_cursor()?;
        Ok(())
    }
    pub fn purge(&mut self) -> Result<(), std::io::Error> {
        Terminal::purge()?;
        Ok(())
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
