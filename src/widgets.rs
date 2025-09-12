//! Provides the [Widget] trait for rendering self-contained elements to the terminal.
//!
//! The most basic examples of widgets provided by `tpop` include [Span], [Line], [Text], and
//! [TextBox].

use std::io::Write;

use crate::{layout::Width, terminal::Terminal};

mod line;
mod span;
mod text;

pub use span::Span;

/// [Widget] is the common trait for renderable widgets.
pub trait Widget {
    /// Render the [Widget] to the given [Width] using the provided [Terminal].
    fn render<W: Write>(&self, width: &Width, terminal: &mut Terminal<W>);
}

/// [StatefulWidget] is similar to [Widget] but it also takes an associated state.  
///
/// This can be used to render more complex interactive elements like multi-selects and progress bars.
pub trait StatefulWidget {
    /// The type of the state of the widget
    type State;

    /// Render the [StatefulWidget] to the given [Width] using the provided [Terminal] and state.
    ///
    /// Should not be called directly unless you want to render the widget a single time. Use
    /// [render_loop](Terminal) instead.
    fn render<W: Write>(
        &self,
        width: &Width,
        terminal: &mut Terminal<W>,
        state: &Self::State,
    ) -> bool;
}
