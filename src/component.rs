//! Provides the [Component] trait, the building block of rendering in `tdrop`.
//!
//! It also provides a number of built-in components, particularly for text rendering.

use std::io::Write;

use crate::{backend::Backend, layout::Width, terminal::Terminal};

/// Component is the building block of `tdrop` rendering
pub trait Component {
    /// Render the component to the terminal
    fn render<B: Backend>(self, width: Width, term: &mut Terminal<B>);
}
