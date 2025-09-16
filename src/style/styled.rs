use std::fmt::Display;

use crate::style::{AsStyle, AsStyleMut, Style};

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
