//! Provides the [Component] trait, the building block of rendering in `tdrop`.
//!
//! It also provides a number of built-in components, particularly for text rendering.

use std::io::Write;

use crate::{layout::Width, terminal::Terminal};

mod span;
pub use span::Span;

/// Component is the building block of `tdrop` rendering
pub trait Component {
    /// Render the component to the terminal
    fn render<W: Write>(self, width: Width, term: &mut Terminal<W>);
}
