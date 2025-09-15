use std::io::Write;

use crossterm::cursor::MoveToNextLine;
use crossterm::execute;
use crossterm::style::Print;
use crossterm::style::SetStyle;
use crossterm::terminal::ScrollUp;

use crate::terminal::TerminalInput;
use crate::{component::Component, layout::Width};

pub struct Options {
    /// Whether the terminal should respect CTRL+C
    respect_exit: bool,
}

/// An abstraction over output through a given backend
pub struct Terminal<W: Write> {
    handle: W,
    width: u16,
}

impl<W: Write> Terminal<W> {
    /// Create a new terminal with the given handle (implementing [Write]).
    /// Returns none if terminal width cannot be retrieved
    pub fn new(handle: W) -> Option<Self> {
        let width = crossterm::terminal::size().map(|w| w.0);
        if let Ok(width) = width {
            return Some(Self { handle, width });
        }

        None
    }

    pub fn render_component<C: Component>(&mut self, comp: C, width: Width) {
        comp.render(width, self);
    }

    pub fn print<I: TerminalInput>(&mut self, content: I) {
        let style = content.style();
        let content = content.content();

        execute!(self.handle, SetStyle(style.into()), Print(content));
    }

    pub fn println<I: TerminalInput>(&mut self, content: I) {
        let style = content.style();
        let content = content.content();

        execute!(
            self.handle,
            SetStyle(style.into()),
            Print(content),
            MoveToNextLine(1),
            ScrollUp(1)
        );
    }

    pub fn width(&self) -> u16 {
        self.width
    }
}

impl<W: Write> Write for Terminal<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.handle.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.handle.flush()
    }
}
