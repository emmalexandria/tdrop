//! Provides the [Component] trait, the building block of rendering in `tdrop`.
//!
//! It also provides a number of built-in components, particularly for text rendering.

use std::io::Write;

use crate::{
    backend::Backend,
    buffer::Buffer,
    layout::{Rect, Width},
    terminal::Terminal,
};

/// Component is the building block of `tdrop` rendering
pub trait Component {
    /// Render the component to the terminal
    fn render(self, area: Rect, buffer: &mut Buffer);
}

pub trait StatefulComponent {
    type State: Sized;

    fn render(self, area: Rect, buffer: &mut Buffer, state: &mut Self::State);
}
