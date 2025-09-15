//! Provides types for representing output styles
//!
//! This module was designed around simplicity, and as such has limited capability to support
//! arbitrary theming needs. Larger applications with complex theming needs should supplant this
//! module with their own implementation.
//!
//! The primary part of this module is [Theme], which is composed of
//! [ThemeStyles](styles::ThemeStyles) and a collection of [Template](templates::Template).     

use crate::style::{AdaptiveColor, Color};

/// Returns if the terminal background is light. If this cannot be determined, return false.
pub fn is_light() -> bool {
    terminal_light::luma().map_or(false, |luma| luma > 0.6)
}

/// A theme represents a set of output colours which adapt to the terminal background
/// ([AdaptiveColor]).
///
/// The primary purpose of [Theme] is to be used for styling the widgets provided by
/// `tdrop`, however it can also be used by application code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Theme {
    /// The primary [AdaptiveColor] of the theme.
    pub primary: AdaptiveColor,
    /// The secondary [AdaptiveColor] of the theme.
    pub secondary: AdaptiveColor,
    /// The error [AdaptiveColor] of the theme.
    pub error: AdaptiveColor,
    /// The warning [AdaptiveColor] of the theme.
    pub warning: AdaptiveColor,
    /// The success [AdaptiveColor] of the theme.
    pub success: AdaptiveColor,
    /// The info [AdaptiveColor] of the theme.
    pub info: AdaptiveColor,
    /// The default [AdaptiveColor] of the theme, usually the same as no colours applied.
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

impl Theme {
    /// A theme with no colours applied
    pub const EMPTY: Self = Self::empty();
    /// Create a new theme with the given set of [AdaptiveColor].
    ///
    /// This should usually not be called directly, but is provided for posterity. Prefer to use
    /// either [Default] for a reasonably okay default theme or use [EMPTY](Self::EMPTY) to get a
    /// theme you can build from the ground up.
    pub const fn new(
        primary: AdaptiveColor,
        secondary: AdaptiveColor,
        error: AdaptiveColor,
        warning: AdaptiveColor,
        success: AdaptiveColor,
        info: AdaptiveColor,
        default: AdaptiveColor,
    ) -> Self {
        Self {
            primary,
            secondary,
            error,
            warning,
            success,
            info,
            default,
        }
    }

    /// Create an empty theme
    pub const fn empty() -> Self {
        Self::new(
            AdaptiveColor::EMPTY,
            AdaptiveColor::EMPTY,
            AdaptiveColor::EMPTY,
            AdaptiveColor::EMPTY,
            AdaptiveColor::EMPTY,
            AdaptiveColor::EMPTY,
            AdaptiveColor::EMPTY,
        )
    }

    /// Set the primary [AdaptiveColor] of the theme.
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn primary<C: Into<AdaptiveColor>>(mut self, color: C) -> Self {
        self.primary = color.into();
        self
    }

    /// Set the secondary [AdaptiveColor] of the theme.
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn secondary<C: Into<AdaptiveColor>>(mut self, color: C) -> Self {
        self.secondary = color.into();
        self
    }

    /// Set the error [AdaptiveColor] of the theme.
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn error<C: Into<AdaptiveColor>>(mut self, color: C) -> Self {
        self.error = color.into();
        self
    }

    /// Set the warning [AdaptiveColor] of the theme.
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn warning<C: Into<AdaptiveColor>>(mut self, color: C) -> Self {
        self.warning = color.into();
        self
    }

    /// Set the success [AdaptiveColor] of the theme.
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn success<C: Into<AdaptiveColor>>(mut self, color: C) -> Self {
        self.success = color.into();
        self
    }

    /// Set the info [AdaptiveColor] of the theme.
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn info<C: Into<AdaptiveColor>>(mut self, color: C) -> Self {
        self.info = color.into();
        self
    }

    /// Set the default [AdaptiveColor] of the theme.
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn default_color<C: Into<AdaptiveColor>>(mut self, color: C) -> Self {
        self.default = color.into();
        self
    }
}
