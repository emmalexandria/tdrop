//! Implements the [Terminal] abstraction over output.

mod viewport;
pub use viewport::Viewport;

mod terminal;
pub use terminal::Terminal;

mod frame;
pub use frame::Frame;
