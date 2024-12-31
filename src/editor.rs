mod handler;
mod viewer;
use crossterm::event;
use handler::terminal::Terminal;
use std::io::Write;
use viewer::Viewer;

#[derive(Debug)]
enum Mode {
    Normal,
    Insert,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Normal
    }
}
#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    mode: Mode,
    pub cursor_x: u16,
    pub cursor_y: u16,
    cols: u16,
    rows: u16,
    view: Viewer,
}

impl Editor {
    pub fn default() -> Self {
        let size = Terminal::get_size().unwrap();
        Editor {
            should_quit: false,
            mode: Mode::Normal,
            cursor_x: 0,
            cursor_y: 0,
            cols: size.0,
            rows: size.1,
            view: Viewer::default(),
        }
    }
    fn handle_normal_mode(&mut self) -> Result<(), std::io::Error> {
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
                crossterm::event::KeyCode::Char('i') => self.mode = Mode::Insert,
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
        Ok(())
    }
    fn handle_insert_mode(&mut self) -> Result<(), std::io::Error> {
        match crossterm::event::read()? {
            crossterm::event::Event::Key(event) => match event.code {
                crossterm::event::KeyCode::Esc => {
                    self.mode = Mode::Normal;
                }
                event::KeyCode::Backspace => todo!(),
                event::KeyCode::Enter => todo!(),
                event::KeyCode::Left => todo!(),
                event::KeyCode::Right => todo!(),
                event::KeyCode::Up => todo!(),
                event::KeyCode::Down => todo!(),
                event::KeyCode::Home => todo!(),
                event::KeyCode::End => todo!(),
                event::KeyCode::PageUp => todo!(),
                event::KeyCode::PageDown => todo!(),
                event::KeyCode::Tab => todo!(),
                event::KeyCode::BackTab => todo!(),
                event::KeyCode::Delete => todo!(),
                event::KeyCode::Insert => todo!(),
                event::KeyCode::F(_) => todo!(),
                event::KeyCode::Char(_) => todo!(),
                event::KeyCode::Null => todo!(),
                event::KeyCode::Esc => todo!(),
                event::KeyCode::CapsLock => todo!(),
                event::KeyCode::ScrollLock => todo!(),
                event::KeyCode::NumLock => todo!(),
                event::KeyCode::PrintScreen => todo!(),
                event::KeyCode::Pause => todo!(),
                event::KeyCode::Menu => todo!(),
                event::KeyCode::KeypadBegin => todo!(),
                event::KeyCode::Media(media_key_code) => todo!(),
                event::KeyCode::Modifier(modifier_key_code) => todo!(),
            },
            event::Event::FocusGained => todo!(),
            event::Event::FocusLost => todo!(),
            event::Event::Key(key_event) => todo!(),
            event::Event::Mouse(mouse_event) => todo!(),
            event::Event::Paste(_) => todo!(),
            event::Event::Resize(_, _) => todo!(),
        }
        Ok(())
    }
    fn handle_mode(&mut self) -> Result<(), std::io::Error> {
        match self.mode {
            Mode::Normal => {
                self.handle_normal_mode()?;
            }
            Mode::Insert => {
                self.handle_insert_mode()?;
            }
        }
        Ok(())
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
            Terminal::draw_statusline(
                self.rows,
                self.cols,
                &self.cursor_x,
                &self.cursor_y,
                &self.mode,
            )?;
            Terminal::move_cursor(self.cursor_x, self.cursor_y)?;
            self.handle_mode()?;
            if self.should_quit {
                break;
            }
            std::io::stdout().flush()?;
        }
        self.purge()?;
        Ok(())
    }
}
