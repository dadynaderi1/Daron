use crossterm::{self, style::Stylize, QueueableCommand};
use std::io::Write;
pub struct Terminal;
pub struct Position(pub u16, pub u16);

impl Position {
    pub fn default() -> Self {
        Self(0, 0)
    }
    pub fn terminal_size() -> Self {
        let size = crossterm::terminal::size().unwrap();
        Self(size.0, size.1)
    }
}
// TODO:: Handling errors with custom types
impl Terminal {
    pub fn initialize() -> Result<(), std::io::Error> {
        crossterm::terminal::enable_raw_mode()?;
        Self::queue_command(crossterm::style::SetBackgroundColor(Self::color(
            34, 36, 54,
        )?))?;
        Self::queue_command(crossterm::terminal::EnterAlternateScreen)?;
        Self::clear_screen()?;
        Self::move_cursor(0, 0)?;
        //std::io::stdout().flush()?;
        Ok(())
    }
    pub fn purge() -> Result<(), std::io::Error> {
        Self::queue_command(crossterm::terminal::LeaveAlternateScreen)?;
        crossterm::terminal::disable_raw_mode()?;
        //std::io::stdout().flush()?;
        Ok(())
    }
    pub fn clear_screen() -> Result<(), std::io::Error> {
        Self::queue_command(crossterm::terminal::Clear(
            crossterm::terminal::ClearType::All,
        ))?;
        //std::io::stdout().flush()?;
        Ok(())
    }
    pub fn print(string: &str) -> Result<(), std::io::Error> {
        Self::queue_command(crossterm::style::Print(string))?;
        Ok(())
    }
    pub fn move_cursor(cursor_x: u16, cursor_y: u16) -> Result<(), std::io::Error> {
        Self::queue_command(crossterm::cursor::MoveTo(cursor_x, cursor_y))?;
        std::io::stdout().flush()?;
        Ok(())
    }
    pub fn get_size() -> Result<(u16, u16), std::io::Error> {
        crossterm::terminal::size()
    }
    pub fn hide_cursor() -> Result<(), std::io::Error> {
        Self::queue_command(crossterm::cursor::Hide)?;
        //std::io::stdout().flush()?;
        Ok(())
    }
    pub fn show_cursor() -> Result<(), std::io::Error> {
        Self::queue_command(crossterm::cursor::Show)?;
        //std::io::stdout().flush()?;
        Ok(())
    }
    pub fn draw_statusline(
        last_row: u16,
        last_col: u16,
        cursor_x: &u16,
        cursor_y: &u16,
        mode: &crate::editor::Mode,
    ) -> Result<(), std::io::Error> {
        let mode_name = format!("  {:?} ", mode);
        let file = "  src/helloWorld.txt  ".to_string();
        let cursor_position = format!(" {}:{} ", cursor_x, cursor_y);
        let file_width = last_col - mode_name.len() as u16 - cursor_position.len() as u16 - 2;
        let seperator = "\u{e0bc}".to_string();
        let reverse_seperator = "\u{e0be}".to_string();
        Self::move_cursor(0, last_row)?;
        Self::queue_command(crossterm::style::PrintStyledContent(
            mode_name
                .to_uppercase()
                .bold()
                .with(crossterm::style::Color::Black)
                .on(Self::color(130, 170, 255)?),
        ))?;
        Self::queue_command(crossterm::style::PrintStyledContent(
            seperator
                .with(Self::color(130, 170, 255)?)
                .on(Self::color(30, 32, 48)?),
        ))?;
        Self::queue_command(crossterm::style::PrintStyledContent(
            format!("{:<width$}", file, width = file_width as usize)
                .white()
                .bold()
                .on(Self::color(30, 32, 48)?),
        ))?;
        Self::queue_command(crossterm::style::PrintStyledContent(
            reverse_seperator
                .with(Self::color(168, 139, 223)?)
                .on(Self::color(30, 32, 48)?),
        ))?;
        Self::queue_command(crossterm::style::PrintStyledContent(
            cursor_position.black().on(Self::color(168, 139, 223)?),
        ))?;
        std::io::stdout().flush()?;
        Ok(())
    }
    fn color(r: u8, g: u8, b: u8) -> Result<crossterm::style::Color, std::io::Error> {
        Ok(crossterm::style::Color::Rgb { r, g, b })
    }
    fn queue_command<T: crossterm::Command>(command: T) -> Result<(), std::io::Error> {
        std::io::stdout().queue(command)?;
        Ok(())
    }
}
