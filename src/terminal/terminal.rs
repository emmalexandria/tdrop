use std::io::{Stdout, Write};

use crossterm::{
    cursor::{MoveToColumn, MoveToNextLine, MoveToPreviousLine},
    style::Print,
    terminal::{Clear, ScrollDown, ScrollUp},
    ExecutableCommand, QueueableCommand,
};
use unicode_segmentation::UnicodeSegmentation;

use crate::{layout::Width, terminal::TerminalInput};

/// [Terminal] is an abstraction over the terminal for use by widgets and applications.
///
/// [Terminal] holds any object which implements [Write], usually [Stdout] or
/// [Stderr](std::io::Stderr).
///
/// ## Examples
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Terminal<W: Write> {
    handle: W,
    /// The width of the output. Equivalent to terminal width if [None].
    pub width: Width,
}

impl Default for Terminal<Stdout> {
    fn default() -> Self {
        Self {
            handle: std::io::stdout(),
            width: Width::default(),
        }
    }
}

impl<W: Write> Terminal<W> {
    /// Create a new [Terminal] with something implementing the [Write] trait.
    pub fn new(handle: W) -> Self {
        Self {
            handle,
            width: Width::default(),
        }
    }

    /// Set the width of [Terminal] to the width of the actual terminal. Will ppanic if the width
    /// of the terminal cannot be retrieved.
    pub fn term_width(self) -> Self {
        if let Ok(s) = crossterm::terminal::size() {
            return self.width(s.0);
        }

        panic!("Failed to retreive terminal size");
    }

    /// Set the handle of the [Terminal] (reccommended: [Stdout] or [Stderr](std::io::Stderr))
    pub fn handle(mut self, handle: W) -> Self {
        self.handle = handle;
        self
    }

    /// Set the width of the [Terminal]
    pub fn width(mut self, width: u16) -> Self {
        self.width = Width::new(0, width);
        self
    }

    /// Print to the terminal. Will truncate text over the terminal's width.
    pub fn print<I: TerminalInput>(&mut self, text: I) -> std::io::Result<()> {
        self.printn(text, self.width.width as usize, true)?;
        Ok(())
    }

    /// Print to the terminal and insert a new line. Will truncate text over the terminal's width.
    pub fn println<I: TerminalInput>(&mut self, text: I) -> std::io::Result<()> {
        self.printn(text, self.width.width as usize, false)?
            .queue(ScrollUp(1))?
            .queue(MoveToNextLine(1))?;

        self.handle.flush()?;

        Ok(())
    }

    /// Prints n characters (graphemes) to the terminal, optionally flushing afterwards. Can be
    /// used to override the terminal width.
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

    /// Create a new line
    pub fn newline(&mut self) -> std::io::Result<()> {
        self.handle
            .queue(MoveToNextLine(1))?
            .queue(ScrollUp(1))?
            .flush()?;

        Ok(())
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

    /// Move the cursor to the given column
    pub fn move_to(&mut self, column: u16) -> std::io::Result<()> {
        self.handle.execute(MoveToColumn(column))?;
        Ok(())
    }

    /// Move the cursor to the beginning of the current line
    pub fn move_to_start(&mut self) -> std::io::Result<()> {
        self.move_to(0)
    }

    /// Clears n lines from the bottom of the terminal
    pub fn clear_n(&mut self, n: u16) -> std::io::Result<()> {
        self.handle
            .queue(MoveToPreviousLine(n))?
            .queue(Clear(crossterm::terminal::ClearType::FromCursorDown))?;
        Ok(())
    }

    /// Flush queued changes to the handle
    pub fn flush(&mut self) -> std::io::Result<()> {
        self.handle.flush()
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
