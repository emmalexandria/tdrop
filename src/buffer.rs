//! Implements the [Buffer] and [Cell] types for outputting to the terminal.
//!
//! These types are primarily used internally by widgets, although you will need to instantiate
//! them yourself occassionally.

mod buffer;
mod cell;
mod diff;

pub use buffer::Buffer;
pub use cell::Cell;

/// Returns a [Buffer] with the given `width` or the width of the terminal if not passed.
///
/// Can fail and return [None] if a width is not passed and it fails to retrieve the terminal
/// width.
pub fn term_buffer(width: Option<u16>) -> Option<Buffer> {
    todo!()
}
