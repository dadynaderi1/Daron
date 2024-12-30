use std::io::Write;

use crossterm::{self, QueueableCommand};
pub struct Terminal;
// TODO:: Handling errors with custom types
impl Terminal {
    pub fn initialize() -> Result<(), std::io::Error> {
        crossterm::terminal::enable_raw_mode()?;
        std::io::stdout().queue(crossterm::terminal::EnterAlternateScreen)?;
        Self::clear_screen()?;
        Self::move_cursor(0, 0)?;
        //std::io::stdout().flush()?;
        Ok(())
    }
    pub fn purge() -> Result<(), std::io::Error> {
        std::io::stdout().queue(crossterm::terminal::LeaveAlternateScreen)?;
        crossterm::terminal::disable_raw_mode()?;
        //std::io::stdout().flush()?;
        Ok(())
    }
    pub fn clear_screen() -> Result<(), std::io::Error> {
        std::io::stdout().queue(crossterm::terminal::Clear(
            crossterm::terminal::ClearType::All,
        ))?;
        //std::io::stdout().flush()?;
        Ok(())
    }
    pub fn move_cursor(cursor_x: u16, cursor_y: u16) -> Result<(), std::io::Error> {
        std::io::stdout().queue(crossterm::cursor::MoveTo(cursor_x, cursor_y))?;
        std::io::stdout().flush()?;
        Ok(())
    }
    pub fn get_size() -> Result<(u16, u16), std::io::Error> {
        crossterm::terminal::size()
    }
    pub fn hide_cursor() -> Result<(), std::io::Error> {
        std::io::stdout().queue(crossterm::cursor::Hide)?;
        //std::io::stdout().flush()?;
        Ok(())
    }
    pub fn show_cursor() -> Result<(), std::io::Error> {
        std::io::stdout().queue(crossterm::cursor::Show)?;
        //std::io::stdout().flush()?;
        Ok(())
    }
}
