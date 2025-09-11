use std::io::{Stdout, Write};

use crossterm::{
    cursor::{MoveToNextLine, MoveToPreviousLine},
    style::Print,
    terminal::{Clear, ScrollDown, ScrollUp},
    ExecutableCommand, QueueableCommand,
};
use unicode_segmentation::UnicodeSegmentation;

use crate::terminal::TerminalInput;

/// [Terminal] is an abstraction over the terminal for use by widgets
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Terminal<W: Write> {
    handle: W,
    /// The width of the output. Equal to terminal width if [None].
    pub width: Option<u16>,
}

impl Default for Terminal<Stdout> {
    fn default() -> Self {
        Self {
            handle: std::io::stdout(),
            width: crossterm::terminal::size().ok().map(|w| w.0),
        }
    }
}

impl<W: Write> Terminal<W> {
    /// Create a new [Terminal] with something implementing the [Write] trait.
    pub const fn new(handle: W, width: Option<u16>) -> Self {
        Self { handle, width }
    }

    /// Set the handle of the [Terminal] (reccommended: [Stdout] or [Stderr](std::io::Stderr))
    pub fn handle(mut self, handle: W) -> Self {
        self.handle = handle;
        self
    }

    /// Set the width of the [Terminal]
    pub fn width(mut self, width: u16) -> Self {
        self.width = Some(width);
        self
    }

    /// Print to the terminal
    pub fn print<I: TerminalInput>(&mut self, text: I) -> std::io::Result<()> {
        self.printn(text, usize::MAX, true)?;
        Ok(())
    }

    /// Print to the terminal and insert a new line
    pub fn println<I: TerminalInput>(&mut self, text: I) -> std::io::Result<()> {
        self.printn(text, usize::MAX, false)?
            .queue(ScrollUp(1))?
            .queue(MoveToNextLine(1))?;

        self.handle.flush()?;

        Ok(())
    }

    /// Prints n characters (graphemes) to the terminal, optionally flushing afterwards
    pub fn printn<I: TerminalInput>(
        &mut self,
        text: I,
        n: usize,
        flush: bool,
    ) -> std::io::Result<&mut W> {
        let first_n: String;
        if text.content().len() > n {
            first_n = text
                // Get text content
                .content()
                // Split into graphemes
                .graphemes(true)
                // Take the first n graphemes
                .take(n)
                // Collect into a vec of strings
                .collect::<Vec<&str>>()
                // Join into a single String
                .join("");
        } else {
            first_n = text.content();
        }

        let res = self.handle.queue(Print(text.style().apply(first_n)))?;

        if flush {
            res.flush()?;
        }

        Ok(res)
    }

    /// Scroll the terminal n lines. n > 0 will scroll up (create blank lines), n < 0 will scroll
    /// down (delete lines). n == 0 will do nothing
    pub fn scroll(&mut self, n: i32) -> std::io::Result<()> {
        if n > 0 {
            self.handle.execute(ScrollUp(1))?;
        } else if n < 0 {
            self.handle.execute(ScrollDown(1))?;
        }

        Ok(())
    }

    /// Clears n lines from the bottom of the terminal
    pub fn clear_n(&mut self, n: u16) -> std::io::Result<()> {
        self.handle
            .queue(MoveToPreviousLine(n))?
            .queue(Clear(crossterm::terminal::ClearType::FromCursorDown))?;
        Ok(())
    }

    /// Get a reference to the handle
    pub fn get_handle(&self) -> &W {
        &self.handle
    }

    /// Get a mutable reference to the handle
    pub fn get_handle_mut(&mut self) -> &mut W {
        &mut self.handle
    }
}
