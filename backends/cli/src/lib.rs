use fgui::*;
use std::io::stdout;

pub struct CliBackend {
    rect: Rect,
}

impl CliBackend {
    pub fn print(&mut self, x: usize, y: usize, message: &str) {
        crossterm::execute!(
            stdout(),
            crossterm::cursor::MoveTo(x as u16, y as u16),
            crossterm::style::Print(message),
        ).unwrap();
    }
}

impl Backend for CliBackend {
    fn init() -> Self {
        let (width, height) = crossterm::terminal::size().unwrap();
        crossterm::terminal::enable_raw_mode().unwrap();

        crossterm::execute!(
            stdout(),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
            crossterm::cursor::Hide,
        ).unwrap();

        let (x, y) = crossterm::cursor::position().unwrap();

        CliBackend {
            rect: Rect::sized(x as usize, y as usize, width as usize, height as usize)
        }
    }

    fn clear(&mut self) -> Area<Self> {
        Area::new(self.rect.clone(), self)
    }
}

pub trait CliArea {
    fn print(&mut self, message: &str);
}

impl<'a> CliArea for Area<'a, CliBackend> {
    fn print(&mut self, message: &str) {
        self.backend.print(self.area.top_left.0, self.area.top_left.1, message)
    }
}
