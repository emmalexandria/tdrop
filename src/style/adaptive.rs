use crate::{style::Color, theme::is_light};

/// Represents a [Color] which is different based on if the terminal background is light or dark.
/// Primarily for use with [Theme].
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AdaptiveColor {
    /// The light [Color] of the adaptive color.
    pub light: Color,
    /// The dark [Color] of the adaptive color.
    pub dark: Color,
}

impl AdaptiveColor {
    /// Get the applicable color based on the terminal background. Will return the dark color if
    /// the background cannot be determined.
    pub fn get(&self) -> Color {
        match is_light() {
            true => self.light,
            false => self.dark,
        }
    }

    /// Set the dark style of the adaptive style and return the modified value
    #[must_use = "moves value of self and returns the modified value"]
    pub fn with_dark<C: Into<Color>>(mut self, dark: C) -> Self {
        self.dark = dark.into();
        self
    }

    /// Set the light style of the adaptive style and return the modified value
    #[must_use = "moves value of self and returns the modified value"]
    pub fn with_light<C: Into<Color>>(mut self, light: C) -> Self {
        self.light = light.into();
        self
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
