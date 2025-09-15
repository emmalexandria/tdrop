use std::fmt::Display;

use crate::style::{style, AsStyle, AsStyleMut, Attribute, Color, Style, StyledString};

/// The [Stylize] trait is used to change the styles of both [Style](super::Style) and any
/// given variable which implements [Display](std::fmt::Display).
pub trait Stylize: Sized {
    /// The styled type
    type Styled: AsStyle + AsStyleMut;

    /// Return the styled type
    fn stylize(self) -> Self::Styled;

    /// Change the foreground [Color]
    fn with(self, color: Color) -> Self::Styled {
        let mut styled = self.stylize();
        styled.as_style_mut().fg = Some(color);
        styled
    }

    /// Change the background [Color]
    fn on<C: Into<Color>>(self, color: C) -> Self::Styled {
        let mut styled = self.stylize();
        styled.as_style_mut().bg = Some(color.into());
        styled
    }

    /// Change the underline [Color]
    fn underline<C: Into<Color>>(self, color: C) -> Self::Styled {
        let mut styled = self.stylize();
        styled.as_style_mut().underline = Some(color.into());
        styled
    }

    /// Set an [Attribute]
    fn attribute<A: Into<Attribute>>(self, attribute: A) -> Self::Styled {
        let mut styled = self.stylize();
        styled.as_style_mut().attributes.set(attribute.into());
        styled
    }

    /// Set the color to [Rest](Color::Reset)
    fn reset(self) -> Self::Styled {
        self.with(Color::Reset)
    }
}

impl Stylize for Style {
    type Styled = Self;

    #[inline]
    fn stylize(self) -> Self::Styled {
        self
    }
}

impl<D: Display> Stylize for StyledString<D> {
    type Styled = StyledString<D>;

    fn stylize(self) -> Self::Styled {
        self
    }
}

impl Stylize for String {
    type Styled = StyledString<Self>;

    fn stylize(self) -> Self::Styled {
        style(self)
    }
}

impl Stylize for &str {
    type Styled = StyledString<Self>;

    fn stylize(self) -> Self::Styled {
        style(self)
    }
}

impl Stylize for char {
    type Styled = StyledString<Self>;

    fn stylize(self) -> Self::Styled {
        style(self)
    }
}
