//! Provides the [Component] trait, the building block of rendering in `tdrop`.

use std::io::Write;

use crate::terminal::Terminal;

/// Component is the building block of `tdrop` rendering
pub trait Component {
    /// Render the component to the terminal
    fn render<W: Write>(self, term: &mut Terminal<W>);
}

