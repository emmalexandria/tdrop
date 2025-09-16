use crate::{
    style::is_light,
    style::{Color, Style},
};

/// Represents a [Color] which is different based on if the terminal background is light or dark.
///
/// Primarily for use with [Theme](crate::theme::Theme) and objects implementing
/// [Widget](crate::widgets::Widget).
///
/// Note that throughout the codebase, [AdaptiveColor] is supported wherever color is through
/// [Into] generics.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AdaptiveColor {
    /// The light [Color] of the adaptive color.
    pub light: Color,
    /// The dark [Color] of the adaptive color.
    pub dark: Color,
}

impl AdaptiveColor {
    /// An adaptive color where both [Color] values are set to [Reset](Color::Reset).
    pub const EMPTY: Self = Self::new(Color::Reset, Color::Reset);

    /// Create a new [AdaptiveColor] with the given light and dark [Color].
    pub const fn new(light: Color, dark: Color) -> Self {
        Self { light, dark }
    }

    /// Get the applicable color based on the terminal background. Will return the dark color if
    /// the background cannot be determined.
    pub fn get(&self) -> Color {
        match is_light() {
            true => self.light,
            false => self.dark,
        }
    }

    /// Set the dark [Color] of the adaptive color and return the modified value.
    #[must_use = "moves value of self and returns the modified value"]
    pub fn dark<C: Into<Color>>(mut self, dark: C) -> Self {
        self.dark = dark.into();
        self
    }

    /// Set the light [Color] of the adaptive color and return the modified value.
    #[must_use = "moves value of self and returns the modified value"]
    pub fn light<C: Into<Color>>(mut self, light: C) -> Self {
        self.light = light.into();
        self
    }

    /// Set the dark [Color] of the adaptive color.
    pub fn set_dark<C: Into<Color>>(&mut self, dark: C) {
        self.dark = dark.into();
    }

    /// Set the light [Color] of the adaptive color.
    pub fn set_light<C: Into<Color>>(&mut self, light: C) {
        self.light = light.into();
    }

    /// Convert the [AdaptiveColor] to a [Style] where it is the foreground.
    pub fn as_fg(&self) -> Style {
        Style::new().fg(self.get())
    }

    /// Convert the [AdaptiveColor] to a [Style] where it is the background.
    pub fn as_bg(&self) -> Style {
        Style::new().bg(self.get())
    }

    /// Convert the [AdaptiveColor] to a [Style] where it is the underline color.
    pub fn as_underline(&self) -> Style {
        Style::new().underline(self.get())
    }
}

impl From<Color> for AdaptiveColor {
    fn from(value: Color) -> Self {
        Self {
            light: value,
            dark: value,
        }
    }
}

impl Into<Color> for AdaptiveColor {
    fn into(self) -> Color {
        self.get()
    }
}

impl Into<Style> for AdaptiveColor {
    fn into(self) -> Style {
        self.as_fg()
    }
}
