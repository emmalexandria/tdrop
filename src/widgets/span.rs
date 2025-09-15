use std::{borrow::Cow, fmt::Display};

use unicode_width::UnicodeWidthStr;

use crate::{
    style::{AsStyle, AsStyleMut, Style, StyledString, Stylize},
    widgets::{Line, Widget},
};

/// [Span] is a [Widget] displaying text with a single [Style](crate::style::Style).
///
/// In implementation, a [Span] is functionally a [StyledString].
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Span<'a> {
    /// The content of the span
    pub content: Cow<'a, str>,
    /// The style of the content
    pub style: Style,
}

impl<'a> Span<'a> {
    /// Create a new [Span] with the given content.
    pub fn raw<T>(content: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self {
            content: content.into(),
            style: Style::default(),
        }
    }

    /// Create a new span with the given content and the given [Style].
    pub fn styled<T, S>(content: T, style: S) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Style>,
    {
        Self {
            content: content.into(),
            style: style.into(),
        }
    }

    /// Sets the content of the span.
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn content<T>(mut self, content: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.content = content.into();
        self
    }

    /// Sets the [Style] of the span.
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    /// Patches the [Style] of the span with another style.
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn patch_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = self.style.patch(style.into());
        self
    }

    /// Resets the [Style] of the span.
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn reset_style(mut self) -> Self {
        self.style = Style::default();
        self
    }

    /// Returns the unicode width of the span.
    pub fn width(&self) -> usize {
        UnicodeWidthStr::width(self)
    }
}

impl<'a, D: Display> From<StyledString<D>> for Span<'a> {
    fn from(value: StyledString<D>) -> Self {
        let style = value.style();
        let content = value.content().to_string();
        Self {
            content: content.into(),
            style: *style,
        }
    }
}

impl<'a, T> From<T> for Span<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(value: T) -> Self {
        Self::raw(value.into())
    }
}

impl<'a> core::ops::Add<Self> for Span<'a> {
    type Output = Line<'a>;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl AsStyle for Span<'_> {
    fn as_style(&self) -> &Style {
        &self.style
    }
}

impl AsStyleMut for Span<'_> {
    fn as_style_mut(&mut self) -> &mut Style {
        &mut self.style
    }
}

impl<'a> Stylize for Span<'a> {
    type Styled = Span<'a>;

    fn stylize(self) -> Self::Styled {
        self
    }
}

impl UnicodeWidthStr for Span<'_> {
    fn width(&self) -> usize {
        self.content.width()
    }

    fn width_cjk(&self) -> usize {
        self.content.width()
    }
}

impl<'a> Display for Span<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.style.apply(self.content.to_owned()))
    }
}

impl<'a> Widget for Span<'a> {
    fn render<W: std::io::Write>(
        &self,
        width: &crate::layout::Width,
        terminal: &mut crate::terminal::Terminal<W>,
    ) {
        width.write(self.style.apply(self.content.clone()), terminal);
    }
}
