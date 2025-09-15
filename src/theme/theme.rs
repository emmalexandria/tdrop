use crate::style::{AdaptiveColor, Color};

/// A theme represents a set of output colours, templates, and styles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Theme {
    /// The primary [Color] of the theme.
    pub primary: AdaptiveColor,
    /// The secondary [Color] of the theme.
    pub secondary: AdaptiveColor,
    /// The error [Color] of the theme.
    pub error: AdaptiveColor,
    /// The warning [Color] of the theme.
    pub warning: AdaptiveColor,
    /// The success [Color] of the theme.
    pub success: AdaptiveColor,
    /// The info [Color] of the theme.
    pub info: AdaptiveColor,
    /// The default [Color] of the theme, usually the same as no colours applied.
    pub default: AdaptiveColor,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            primary: Color::Cyan.into(),
            secondary: Color::Green.into(),
            error: Color::Red.into(),
            warning: Color::Yellow.into(),
            success: Color::Green.into(),
            info: Color::Blue.into(),
            default: Color::Reset.into(),
        }
    }
}
