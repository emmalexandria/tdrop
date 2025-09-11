use std::fmt::Display;

use crate::{
    style::{StyledString, Stylize},
    widgets::Widget,
};

/// A [Span] is a [Widget] displaying text with a unified style.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Span {
    content: StyledString<String>,
}

impl Span {
    /// Create a new [Span] with the given content.
    ///
    /// Please note that within the type system, it is entirely possible to create a [Span] with
    /// [StyledContent]
    pub fn new<S: AsRef<str>>(content: S) -> Self {
        Self {
            content: content.as_ref().to_string().stylize(),
        }
    }
}

impl<D: Display> From<StyledString<D>> for Span {
    fn from(value: StyledString<D>) -> Self {
        let style = value.style();
        let content = value.content().to_string();
        Self {
            content: style.apply(content),
        }
    }
}

impl Widget for Span {
    fn render<W: std::io::Write>(
        self,
        width: &crate::layout::Width,
        terminal: &mut crate::terminal::Terminal<W>,
    ) {
        width.write(self.content, terminal);
    }
}
