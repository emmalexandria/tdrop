use std::io::{self, Write};

use crate::buffer::Cell;
use crate::layout::{Position, Size};
use crate::style::{Attribute, Attributes, Color};

use crossterm::cursor::{Hide, MoveTo, Show};

use crossterm::style::{
    Attribute as CrosstermAttr, Attributes as CrosstermAttrs, Color as CrosstermColor,
    Colors as CrosstermColors, ContentStyle, Print, SetAttribute, SetBackgroundColor, SetColors,
    SetForegroundColor,
};

use crossterm::terminal::{self, Clear};
use crossterm::{execute, queue};

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

    fn clear(&mut self) -> Result<(), Self::Error>;

    fn size(&self) -> Result<Size, Self::Error>;

    fn flush(&mut self) -> Result<(), Self::Error>;
}

pub struct CrosstermBackend<W: Write> {
    writer: W,
}

impl<W> CrosstermBackend<W>
where
    W: Write,
{
    pub const fn new(writer: W) -> Self {
        Self { writer }
    }

    pub const fn writer(&self) -> &W {
        &self.writer
    }

    pub const fn writer_mut(&mut self) -> &mut W {
        &mut self.writer
    }
}

impl<W> Write for CrosstermBackend<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl<W> Backend for CrosstermBackend<W>
where
    W: Write,
{
    type Error = io::Error;

    fn draw<'a, I>(&mut self, content: I) -> Result<(), Self::Error>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        let mut fg = Color::Reset;
        let mut bg = Color::Reset;
        let mut attrs = Attributes::none();
        let mut last_pos: Option<Position> = None;
        for (x, y, cell) in content {
            if !matches!(last_pos, Some(p) if x == p.x + 1 && y == p.y) {
                queue!(self.writer, MoveTo(x, y))?;
            }
            last_pos = Some(Position { x, y });
            if cell.attributes != attrs {
                let diff = AttributeDiff {
                    from: attrs,
                    to: cell.attributes,
                };
                diff.queue(&mut self.writer)?;
                attrs = cell.attributes;
            }
            if (cell.fg != fg || cell.bg != bg) {
                queue!(
                    self.writer,
                    SetColors(CrosstermColors::new(
                        cell.fg.into_crossterm(),
                        cell.bg.into_crossterm()
                    ))
                )?;
            }

            queue!(self.writer, Print(cell.symbol()))?;
        }

        Ok(())
    }

    fn append_lines(&mut self, n: u16) -> io::Result<()> {
        for _ in 0..n {
            queue!(self.writer, Print("\n"))?;
        }

        self.writer.flush()
    }

    fn hide_cursor(&mut self) -> io::Result<()> {
        execute!(self.writer, Hide)
    }

    fn show_cursor(&mut self) -> io::Result<()> {
        execute!(self.writer, Show)
    }

    fn get_cursor_position(&mut self) -> io::Result<Position> {
        crossterm::cursor::position()
            .map(|(x, y)| Position { x, y })
            .map_err(io::Error::other)
    }

    fn set_cursor_position<P: Into<Position>>(&mut self, position: P) -> io::Result<()> {
        let Position { x, y } = position.into();
        execute!(self.writer, MoveTo(x, y))
    }

    fn clear(&mut self) -> io::Result<()> {
        self.clear_region(ClearType::All)
    }

    fn clear_region(&mut self, clear_type: ClearType) -> io::Result<()> {
        execute!(
            self.writer,
            Clear(match clear_type {
                ClearType::All => crossterm::terminal::ClearType::All,
                ClearType::AfterCursor => crossterm::terminal::ClearType::FromCursorDown,
                ClearType::BeforeCursor => crossterm::terminal::ClearType::FromCursorUp,
                ClearType::CurrentLine => crossterm::terminal::ClearType::CurrentLine,
                ClearType::UntilNewLine => crossterm::terminal::ClearType::UntilNewLine,
            })
        )
    }

    fn size(&self) -> io::Result<Size> {
        let (width, height) = crossterm::terminal::size()?;

        Ok(Size { width, height })
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

pub trait IntoCrossterm<C> {
    fn into_crossterm(self) -> C;
}

pub trait FromCrossterm<C> {
    fn from_crossterm(value: C) -> Self;
}

impl IntoCrossterm<CrosstermColor> for Color {
    fn into_crossterm(self) -> CrosstermColor {
        match self {
            Color::Reset => crossterm::style::Color::Reset,
            Color::Black => crossterm::style::Color::Black,
            Color::Red => crossterm::style::Color::DarkRed,
            Color::Green => crossterm::style::Color::DarkGreen,
            Color::Yellow => crossterm::style::Color::DarkYellow,
            Color::Blue => crossterm::style::Color::DarkBlue,
            Color::Magenta => crossterm::style::Color::DarkMagenta,
            Color::Cyan => crossterm::style::Color::DarkCyan,
            Color::Gray => crossterm::style::Color::Grey,
            Color::DarkGray => crossterm::style::Color::DarkGrey,
            Color::BrightRed => crossterm::style::Color::Red,
            Color::BrightGreen => crossterm::style::Color::Green,
            Color::BrightYellow => crossterm::style::Color::Yellow,
            Color::BrightBlue => crossterm::style::Color::Blue,
            Color::BrightMagenta => crossterm::style::Color::Magenta,
            Color::BrightCyan => crossterm::style::Color::Cyan,
            Color::White => crossterm::style::Color::White,
            Color::Rgb { r, g, b } => crossterm::style::Color::Rgb { r, g, b },
            Color::Indexed(i) => crossterm::style::Color::AnsiValue(i),
        }
    }
}

impl FromCrossterm<CrosstermColor> for Color {
    fn from_crossterm(value: CrosstermColor) -> Self {
        match value {
            crossterm::style::Color::Reset => Self::Reset,
            crossterm::style::Color::Black => Self::Black,
            crossterm::style::Color::DarkGrey => Self::DarkGray,
            crossterm::style::Color::Red => Self::BrightRed,
            crossterm::style::Color::DarkRed => Self::Red,
            crossterm::style::Color::Green => Self::BrightGreen,
            crossterm::style::Color::DarkGreen => Self::Green,
            crossterm::style::Color::Yellow => Self::BrightYellow,
            crossterm::style::Color::DarkYellow => Self::Yellow,
            crossterm::style::Color::Blue => Self::BrightBlue,
            crossterm::style::Color::DarkBlue => Self::Blue,
            crossterm::style::Color::Magenta => Self::BrightMagenta,
            crossterm::style::Color::DarkMagenta => Self::Magenta,
            crossterm::style::Color::Cyan => Self::BrightCyan,
            crossterm::style::Color::DarkCyan => Self::Cyan,
            crossterm::style::Color::White => Self::White,
            crossterm::style::Color::Grey => Self::Gray,
            crossterm::style::Color::Rgb { r, g, b } => Self::Rgb { r, g, b },
            crossterm::style::Color::AnsiValue(i) => Self::Indexed(i),
        }
    }
}

struct AttributeDiff {
    pub from: Attributes,
    pub to: Attributes,
}

impl AttributeDiff {
    fn queue<W>(self, mut w: W) -> io::Result<()>
    where
        W: Write,
    {
        let removed = self.from - self.to;
        if removed.has(Attribute::Reverse) {
            queue!(w, SetAttribute(CrosstermAttr::NoReverse))?;
        }
        if removed.has(Attribute::Bold) || removed.has(Attribute::Dim) {
            queue!(w, SetAttribute(CrosstermAttr::NormalIntensity))?;

            if self.to.has(Attribute::Dim) {
                queue!(w, SetAttribute(CrosstermAttr::Dim))?;
            }

            if self.to.has(Attribute::Bold) {
                queue!(w, SetAttribute(CrosstermAttr::Bold))?;
            }
        }
        if removed.has(Attribute::Italic) {
            queue!(w, SetAttribute(CrosstermAttr::NoItalic))?;
        }
        if removed.has(Attribute::Underlined) {
            queue!(w, SetAttribute(CrosstermAttr::NoUnderline))?;
        }
        if removed.has(Attribute::Strikethrough) {
            queue!(w, SetAttribute(CrosstermAttr::NotCrossedOut))?;
        }
        if removed.has(Attribute::SlowBlink) || removed.has(Attribute::RapidBlink) {
            queue!(w, SetAttribute(CrosstermAttr::NoBlink))?;
        }

        let added = self.to - self.from;
        if added.has(Attribute::Reverse) {
            queue!(w, SetAttribute(CrosstermAttr::Reverse))?;
        }
        if added.has(Attribute::Bold) {
            queue!(w, SetAttribute(CrosstermAttr::Bold))?;
        }
        if added.has(Attribute::Italic) {
            queue!(w, SetAttribute(CrosstermAttr::Italic))?;
        }
        if added.has(Attribute::Dim) {
            queue!(w, SetAttribute(CrosstermAttr::Dim))?;
        }
        if added.has(Attribute::Strikethrough) {
            queue!(w, SetAttribute(CrosstermAttr::CrossedOut))?;
        }
        if added.has(Attribute::SlowBlink) {
            queue!(w, SetAttribute(CrosstermAttr::SlowBlink))?;
        }
        if added.has(Attribute::RapidBlink) {
            queue!(w, SetAttribute(CrosstermAttr::RapidBlink))?;
        }

        Ok(())
    }
}
