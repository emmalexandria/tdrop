//! An implementation of ANSI styling through [Attribute], [Attributes], and
//! [Color].
//!
//! At present, it is practically a re-implementation of the types implemented in
//! crossterm. Rendering is handled through crossterm by converting back into crossterm types. This
//! is inneficient, but it was done to reduce direct use of crossterm by dependent crates and to
//! make supporting multiple backends easier down the road.
//!
//! To create and apply a style:
//! ```
//! use tdrop::style::{Style, Color, Stylize, Attribute};
//!
//! let style = Style::new().with(Color::BrightRed).attribute(Attribute::Bold);
//! let styled_str = style.apply("Hello");
//!
//! // Hello will print in bright red and bold
//! println!("{styled_str}")
//! ```
//! Or to style a string without creating a style first:
//! ```
//! use tdrop::style::{Color, Attribute, Stylize};
//!
//! println!("{}", "hello".with(Color::Rgb{r: 32, g: 50, b: 42}).attribute(Attribute::Italic));
//! ```
//!

mod adaptive;
mod attributes;
mod color;
mod styled;
mod stylize;

pub use adaptive::AdaptiveColor;
pub use attributes::{Attribute, Attributes};
pub use color::Color;
pub use styled::StyledString;
pub use stylize::Stylize;

use std::fmt::Display;

/// Returns whether the terminal background is light
pub fn is_light() -> bool {
    terminal_light::luma().map_or(false, |luma| luma > 0.6)
}

/// Trait for retrieving the style of a type as a reference
pub trait AsStyle {
    /// Get a reference to the type's style
    fn as_style(&self) -> &Style;
}

/// Trait for retrieving the style of the type as a mutable reference.
pub trait AsStyleMut {
    /// Get a mutable reference to the type's style
    fn as_style_mut(&mut self) -> &mut Style;
}

/// Creates a new [StyledString]
pub fn style<D: Display>(val: D) -> StyledString<D> {
    Style::new().apply(val)
}

/// [Style] represents the output styles for a piece of text.
///
/// It includes an optional foreground [Color], background [Color], underline [Color],
/// and applied [Attributes]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Style {
    /// The foreground color of the style if any.
    pub fg: Option<Color>,
    /// The background color of the style if any.
    pub bg: Option<Color>,

    /// The underline color of the style if any.
    pub underline: Option<Color>,

    /// The attributes of the style.
    pub attributes: Attributes,
}

impl AsStyleMut for Style {
    fn as_style_mut(&mut self) -> &mut Style {
        self
    }
}

impl AsStyle for Style {
    fn as_style(&self) -> &Style {
        self
    }
}

impl Style {
    /// Applies the style to a given [Display] implementing type, returning a [StyledString]
    #[inline]
    pub fn apply<D: Display>(self, val: D) -> StyledString<D> {
        StyledString::new(self, val)
    }

    /// Creates an empty [Style]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the foreground [Color] and return the modified value of self.
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn fg<C: Into<Color>>(mut self, color: C) -> Self {
        self.fg = Some(color.into());
        self
    }

    /// Set the background [Color] and return the modified value of self.
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn bg<C: Into<Color>>(mut self, color: C) -> Self {
        self.bg = Some(color.into());
        self
    }

    /// Set the underline [Color] and return the modified value of self.
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn underline<C: Into<Color>>(mut self, color: C) -> Self {
        self.underline = Some(color.into());
        self
    }

    /// Set the [Attributes] and return the modified value of self.
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn attributes(mut self, attributes: Attributes) -> Self {
        self.attributes = attributes;
        self
    }

    /// Set an [Attribute] and return the modified value of self.
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn attribute(mut self, attr: Attribute) -> Self {
        self.attributes.set(attr);
        self
    }

    /// Patch this style with another given style, returning the result
    ///
    /// This method prioritises the colors of `self`, only overriding them if they are not
    /// present. Attributes are extended, meaning any attributes from `other` will be added
    /// to the attributes of the return value if not present, but no attributes will be removed.
    #[must_use = "does not modify self, returns a new value"]
    pub fn patch<S: Into<Style>>(&self, other: S) -> Self {
        let other: Style = other.into();
        Self {
            fg: self.fg.or(other.fg),
            bg: self.bg.or(other.bg),

            underline: self.underline.or(other.underline),

            attributes: self.attributes.patch(other.attributes),
        }
    }
}
