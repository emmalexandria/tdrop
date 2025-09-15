use crate::style::AdaptiveStyle;

/// An enum of the possible theme styles of a [Theme]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThemeStyle {
    /// The primary style
    Primary,
    /// The secondary style
    Secondary,
    /// The error style
    Error,
    /// The warning style
    Warning,
    /// The success style
    Success,
    /// The info style
    Info,
    /// The default style for basic output
    #[default]
    Output,
}

/// A type for storing theme colors and styles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ThemeStyles {
    /// The style of the theme for 'primary' output.
    ///
    /// This is intended to be used for things which should be in some colour associated with your
    /// application.
    pub primary: AdaptiveStyle,
    /// The style of the theme for 'secondary' output.
    ///
    /// This usually contains an accent colour.
    pub secondary: AdaptiveStyle,
    /// The style of the theme for errors.
    pub error: AdaptiveStyle,
    /// The style of the theme for warnings.
    pub warning: AdaptiveStyle,
    /// The style of the theme for success messages.
    pub success: AdaptiveStyle,
    /// The style of the theme for informational messages.
    pub info: AdaptiveStyle,
    /// The style of the theme for standard terminal output.
    pub output: AdaptiveStyle,
}

impl ThemeStyles {
    /// Create a new empty theme where every style is set to the default
    pub fn new() -> Self {
        Self::default()
    }

    /// Set an [AdaptiveStyle] of the theme by the [ThemeStyle] enum.
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn style<S: Into<AdaptiveStyle>>(self, identifier: ThemeStyle, style: S) -> Self {
        match identifier {
            ThemeStyle::Primary => self.primary(style),
            ThemeStyle::Secondary => todo!(),
            ThemeStyle::Error => todo!(),
            ThemeStyle::Warning => todo!(),
            ThemeStyle::Success => todo!(),
            ThemeStyle::Info => todo!(),
            ThemeStyle::Output => todo!(),
        }
    }

    /// Set a given [AdaptiveStyle].
    pub fn set_style<S: Into<AdaptiveStyle>>(&mut self, identifier: ThemeStyle, style: S) {
        match identifier {
            ThemeStyle::Primary => self.primary = style.into(),
            ThemeStyle::Secondary => todo!(),
            ThemeStyle::Error => todo!(),
            ThemeStyle::Warning => todo!(),
            ThemeStyle::Success => todo!(),
            ThemeStyle::Info => todo!(),
            ThemeStyle::Output => todo!(),
        }
    }

    /// Set the primary [AdaptiveStyle] of the theme.
    pub fn set_primary<S: Into<AdaptiveStyle>>(&mut self, style: S) {
        self.primary = style.into();
    }

    /// Set the primary [AdaptiveStyle] of the theme.
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn primary<S: Into<AdaptiveStyle>>(mut self, style: S) -> Self {
        self.primary = style.into();
        self
    }
}
