use std::io::Write;

use crossterm::cursor::MoveToNextLine;
use crossterm::execute;
use crossterm::queue;
use crossterm::style::Print;
use crossterm::style::SetStyle;
use crossterm::terminal::ScrollUp;
use crossterm::Command;
use crossterm::QueueableCommand;

use crate::terminal::TerminalInput;
use crate::{component::Component, layout::Width};

pub struct Options {
    /// Whether the terminal should respect CTRL+C
    respect_exit: bool,
}

/// An abstraction over output through a given backend
pub struct Terminal<W: Write> {
    handle: W,
}

impl<W: Write> Terminal<W> {
    /// Create a new terminal with the given handle (implementing [Write])
    pub fn new(handle: W) -> Self {
        Self { handle }
    }

    pub fn render_component<C: Component>(&mut self, comp: C, width: Width) {
        comp.render(width, self);
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
}

impl<W: Write> Write for Terminal<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.handle.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.handle.flush()
    }
}
