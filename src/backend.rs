use crate::terminal::Cell;

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
}
