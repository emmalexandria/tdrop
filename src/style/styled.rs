use std::fmt::Display;

use crate::{
    style::{AsStyle, AsStyleMut, Style},
    terminal::TerminalInput,
};

/// [StyledString] is a type associating a [Style] with any type which implements [Display].
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct StyledString<D: Display> {
    style: Style,
    content: D,
}

impl<D: Display> StyledString<D> {
    /// Create a new [StyledString].
    #[inline]
    pub fn new(style: Style, content: D) -> Self {
        StyledString { style, content }
    }

    /// Get the content of the [StyledString].
    #[inline]
    pub fn content(&self) -> &D {
        &self.content
    }

    /// Get the [Style] of the [StyledString].
    #[inline]
    pub fn style(&self) -> &Style {
        &self.style
    }

    /// Get a mutable reference to the [Style].
    #[inline]
    pub fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    /// Return the length of the inner content of the [StyledString]
    #[inline]
    pub fn len(&self) -> usize {
        self.content.to_string().len()
    }
}

impl<D: Display> AsStyle for StyledString<D> {
    fn as_style(&self) -> &Style {
        &self.style
    }
}

impl<D: Display> AsStyleMut for StyledString<D> {
    fn as_style_mut(&mut self) -> &mut Style {
        &mut self.style
    }
}

impl<D: Display + Clone> From<crossterm::style::StyledContent<D>> for StyledString<D> {
    fn from(value: crossterm::style::StyledContent<D>) -> Self {
        Self::new(value.style().to_owned().into(), value.content().clone())
    }
}

impl<D: Display> Into<crossterm::style::StyledContent<D>> for StyledString<D> {
    fn into(self) -> crossterm::style::StyledContent<D> {
        crossterm::style::StyledContent::new(self.style.into(), self.content)
    }
}

impl<D: Display + Clone> Into<crossterm::style::StyledContent<D>> for &StyledString<D> {
    fn into(self) -> crossterm::style::StyledContent<D> {
        crossterm::style::StyledContent::new(self.style.into(), self.content.clone())
    }
}

impl<D: Display> TerminalInput for StyledString<D> {
    fn content(&self) -> String {
        self.content.to_string()
    }

    fn style(&self) -> Style {
        self.style
    }
}

impl<D: Display> AsRef<Style> for StyledString<D> {
    fn as_ref(&self) -> &Style {
        &self.style
    }
}

impl<D: Display> AsMut<Style> for StyledString<D> {
    fn as_mut(&mut self) -> &mut Style {
        &mut self.style
    }
}

impl<D: Display + Clone> Display for StyledString<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ct_content: crossterm::style::StyledContent<D> = self.into();
        write!(f, "{ct_content}")
    }
}
