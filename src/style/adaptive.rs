use std::fmt::Display;

use crate::{
    style::{Style, StyledString},
    theme::is_light,
};

/// Represents a style which applies differently based on if the terminal background is light or
/// dark primarily for use with [Theme](crate::theme::Theme)
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AdaptiveStyle(Style, Style);

impl AdaptiveStyle {
    /// Create a new adaptive style with a light and dark style
    pub const fn new(dark: Style, light: Style) -> Self {
        Self(dark, light)
    }

    /// Creates a new adaptive style from a single style, setting both to it.
    pub const fn single(style: Style) -> Self {
        Self(style, style)
    }

    /// Set the dark style of the adaptive style and return the modified value
    #[must_use = "moves value of self and returns the modified value"]
    pub fn with_dark(mut self, dark: Style) -> Self {
        self.0 = dark;
        self
    }

    /// Set the light style of the adaptive style and return the modified value
    #[must_use = "moves value of self and returns the modified value"]
    pub fn with_light(mut self, light: Style) -> Self {
        self.1 = light;
        self
    }

    /// Apply the style creating a [StyledString]
    #[inline]
    pub fn apply<D: Display>(&self, content: D) -> StyledString<D> {
        let style = self.get();
        StyledString::new(style, content)
    }

    /// Get the applicable style based on the terminal background
    #[inline]
    pub fn get(&self) -> Style {
        match is_light() {
            false => self.0,
            true => self.1,
        }
    }
}
