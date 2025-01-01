mod handler;
mod viewer;
use anyhow::Result;
use crossterm::ExecutableCommand;
use handler::terminal::Position;
use handler::terminal::Terminal;
use std::io::Write;
use viewer::Viewer;
#[derive(Debug)]
pub enum Mode {
    Normal,
    Insert,
}
enum Action {
    MoveDown,
    MoveUp,
    MoveLeft,
    MoveRight,
}

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
        let size = Position::terminal_size();
        let cursor_pos = Position::default();
        Editor {
            should_quit: false,
            mode: Mode::Normal,
            cursor_x: cursor_pos.0,
            cursor_y: cursor_pos.1,
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

            crossterm::event::Event::Resize(x, y) => {
                self.cols = x;
                self.rows = y;
            }
            _ => (),
        }
        Ok(())
    }
    fn handle_insert_mode(&mut self) -> Result<(), std::io::Error> {
        match crossterm::event::read()? {
            crossterm::event::Event::Key(event) => match event.code {
                crossterm::event::KeyCode::Char(char) => {
                    std::io::stdout().execute(crossterm::style::Print(char))?;
                    self.cursor_x += 1;
                }
                crossterm::event::KeyCode::Esc => {
                    self.mode = Mode::Normal;
                }
                _ => (),
            },

            _ => (),
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
