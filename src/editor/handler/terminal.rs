use crossterm::{self, ExecutableCommand};
pub struct Terminal;
// TODO:: Handling errors with custom types
impl Terminal {
    pub fn initialize() -> Result<(), std::io::Error> {
        crossterm::terminal::enable_raw_mode()?;
        std::io::stdout().execute(crossterm::terminal::EnterAlternateScreen)?;
        Self::clear_screen()?;
        Self::move_cursor(0, 0)?;
        Ok(())
    }
    pub fn purge() -> Result<(), std::io::Error> {
        std::io::stdout().execute(crossterm::terminal::LeaveAlternateScreen)?;
        crossterm::terminal::disable_raw_mode()?;
        Ok(())
    }
    pub fn clear_screen() -> Result<(), std::io::Error> {
        std::io::stdout().execute(crossterm::terminal::Clear(
            crossterm::terminal::ClearType::All,
        ))?;
        Ok(())
    }
    pub fn move_cursor(cursor_x: u16, cursor_y: u16) -> Result<(), std::io::Error> {
        std::io::stdout().execute(crossterm::cursor::MoveTo(cursor_x, cursor_y))?;
        Ok(())
    }
    pub fn get_size() -> Result<(u16, u16), std::io::Error> {
        crossterm::terminal::size()
    }
}
