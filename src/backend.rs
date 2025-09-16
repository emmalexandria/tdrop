use crate::buffer::Cell;
use crate::layout::{Position, Size};

pub enum ClearType {
    All,
    AfterCursor,
    BeforeCursor,
    CurrentLine,
    UntilNewLine,
}

pub trait Backend {
    type Error: std::error::Error;

    fn draw<'a, I>(&mut self, content: I) -> Result<(), Self::Error>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>;

    fn append_lines(&mut self, n: u16) -> Result<(), Self::Error>;

    fn hide_cursor(&mut self) -> Result<(), Self::Error>;

    fn show_cursor(&mut self) -> Result<(), Self::Error>;

    fn get_cursor_position(&mut self) -> Result<Position, Self::Error>;

    fn set_cursor_position<P: Into<Position>>(&mut self, position: P) -> Result<(), Self::Error>;

    fn clear_region(&mut self, clear_type: ClearType) -> Result<(), Self::Error>;

    fn size(&self) -> Result<Size, Self::Error>;

    fn flush(&mut self) -> Result<(), Self::Error>;
}
