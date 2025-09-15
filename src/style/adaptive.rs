use std::fmt::Display;

use crate::{
    style::{Style, StyledString, Stylize},
    theme::is_light,
};

/// Represents a style which applies differently based on if the terminal background is light or
/// dark primarily for use with [Theme](crate::theme::Theme)
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AdaptiveStyle {
    /// The [Style] that is applied when the terminal background is light.
    pub light: Style,
    /// The [Style] that is applied when the terminal background is dark or cannot be determined.
    pub dark: Style,
}

impl AdaptiveStyle {
    /// Create a new adaptive style with a light and dark style
    pub const fn new(dark: Style, light: Style) -> Self {
        Self { dark, light }
    }

    /// Creates a new adaptive style from a single style, setting both to it.
    pub const fn single(style: Style) -> Self {
        Self {
            dark: style,
            light: style,
        }
    }

    /// Set the dark style of the adaptive style and return the modified value
    #[must_use = "moves value of self and returns the modified value"]
    pub fn with_dark(mut self, dark: Style) -> Self {
        self.dark = dark;
        self
    }

    /// Set the light style of the adaptive style and return the modified value
    #[must_use = "moves value of self and returns the modified value"]
    pub fn with_light(mut self, light: Style) -> Self {
        self.light = light;
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
            false => self.dark,
            true => self.light,
        }
    }
}

impl From<Style> for AdaptiveStyle {
    fn from(value: Style) -> Self {
        Self {
            dark: value,
            light: value,
        }
    }
}

impl From<&Style> for AdaptiveStyle {
    fn from(value: &Style) -> Self {
        Self {
            dark: *value,
            light: *value,
        }
    }
}

impl Into<Style> for AdaptiveStyle {
    fn into(self) -> Style {
        self.get()
    }
}

impl From<crossterm::style::ContentStyle> for AdaptiveStyle {
    fn from(value: crossterm::style::ContentStyle) -> Self {
        let style = value.into();
        Self {
            dark: style,
            light: style,
        }
    }
}
