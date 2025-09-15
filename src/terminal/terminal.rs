use std::io::Write;

/// An abstraction over output through a given backend
pub struct Terminal<W: Write> {
    handle: W,
}

impl<W: Write> Terminal<W> {
    /// Create a new terminal with the given handle (implementing [Write])
    pub fn new(handle: W) -> Self {
        Self { handle }
    }
}
