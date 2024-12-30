use super::handler::terminal::Terminal;
mod buffer;
// Enviromental variables!
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
// End of enviromental variables
pub struct Viewer;

impl Viewer {
    pub fn renderer(&mut self, rows: u16, cols: u16) -> Result<(), std::io::Error> {
        for i in 0..rows + 1 {
            Terminal::move_cursor(0, i)?;
            if i == rows / 2 {
                Self::splash_screen(cols)?;
            } else {
                Terminal::print("~")?;
            }
        }

        Ok(())
    }
    pub fn splash_screen(cols: u16) -> Result<(), std::io::Error> {
        //Terminal::show_cursor().unwrap();
        //Terminal::move_cursor(self.cols / 2 - 5, self.rows / 2).unwrap();
        let mut name = format!("{NAME} editor -- version: {VERSION}");
        let cols = cols as usize;
        let padding = (cols - name.len()) / 2;
        let spaces = " ".repeat(padding - 1);
        name = format!("~{spaces}{name}");
        name.truncate(cols);
        Terminal::print(&name)?;
        Ok(())
        //Terminal::hide_cursor().unwrap();
    }
}
