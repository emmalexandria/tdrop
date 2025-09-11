use crate::layout::Rect;

use super::Cell;

/// [Buffer] is used to output to the terminal.
///
/// It implements a list of [Cell] covering the terminal width.
/// Writing to a [Buffer] does not inherently write to the terminal. Calling
/// `sync` will diff the [Buffer] and the terminal and write changes.
///
/// Although the terminology is the same as `ratatui`, it is crucial to note that this buffer
/// serves a different function. It does not represent an area of the terminal as we are not in raw
/// mode. Rather, it represents the terminal as a whole.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Buffer {
    curr: BufferContent,
    last: BufferContent,
}

impl Buffer {
    /// Syncs the current contents of the buffer to the given output stream
    pub fn sync(&self) -> std::io::Result<()> {
        todo!()
    }

    /// Prints the given text at the end of the buffer and scrolls to the next line.
    pub fn println(&mut self) {
        todo!()
    }

    /// Prints the given text at the end of the buffer
    pub fn print(&mut self) {
        todo!()
    }

    /// Inserts a new line into the buffer and scrolls the terminal
    pub fn newline(&mut self) {
        todo!()
    }

    /// Clears a given [Rect] from the buffer
    ///
    /// This is useful for widgets which need to redraw themselves multiple times such as progress
    /// bars.
    pub fn clear_rect(&self, rect: Rect) {
        todo!()
    }

    /// Clears a given [Rect] from the buffer and scrolls the content below it up.
    ///
    /// This is useful if you want to implement a widget which, for example, displays a
    /// yes/no prompt and collapses after a selection is made.
    pub fn clear_and_scroll(&self, rect: Rect) {}
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
struct BufferContent {
    cells: Vec<Cell>,
}

/// [BufferCursor] stores the position of the cursor relative to the buffer and is responsible for
/// writing and clearing text within the buffer.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
struct BufferCursor {
    rel_x: u16,
    rel_y: u16,
}
