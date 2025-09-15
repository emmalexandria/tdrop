//! Implements the [Terminal] abstraction over output.

mod terminal;
pub use terminal::Terminal;

mod cell;
pub use cell::Cell;

mod input;
pub use input::Input as TerminalInput;
