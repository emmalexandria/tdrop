use std::{borrow::Cow, fmt::Display};

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::{
    buffer::Buffer, layout::rect::Rect, style::PatchStyle, text::grapheme::StyledGrapheme,
    widgets::Widget, Style,
};

#[derive(Default, Clone, Eq, PartialEq)]
pub struct Span<'a> {
    pub style: Style,
    pub content: Cow<'a, str>,
}

impl<'a> Span<'a> {
    pub fn raw<T>(content: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self {
            content: content.into(),
            style: Style::default(),
        }
    }

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

    pub fn content<T>(mut self, content: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.content = content.into();
        self
    }

    pub fn style<S>(mut self, style: S) -> Self
    where
        S: Into<Style>,
    {
        self.style = style.into();
        self
    }

    pub fn patch_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = self.style.patch(style);
        self
    }

    pub fn reset_style(mut self) -> Self {
        self.style = Style::default();
        self
    }

    pub fn width(&self) -> usize {
        UnicodeWidthStr::width(self)
    }

    pub fn styled_graphemes<S: Into<Style>>(
        &'a self,
        base_style: S,
    ) -> impl Iterator<Item = StyledGrapheme<'a>> {
        let style = base_style.into().patch(self.style);
        self.content
            .as_ref()
            .graphemes(true)
            .filter(|g| !g.contains(char::is_control))
            .map(move |g| StyledGrapheme { symbol: g, style })
    }
}

impl UnicodeWidthStr for Span<'_> {
    fn width(&self) -> usize {
        self.content.width()
    }

    fn width_cjk(&self) -> usize {
        self.content.width_cjk()
    }
}

impl<'a, T> From<T> for Span<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(s: T) -> Self {
        Span::raw(s.into())
    }
}

impl Widget for Span<'_> {
    fn render(self, area: Rect) -> Buffer {
        Widget::render(&self, area)
    }
}

impl Widget for &Span<'_> {
    fn render(self, area: Rect) -> Buffer {
        let mut buffer = Buffer::empty(area);

        buffer.set_string(0, 0, self.content.clone(), self.style);

        buffer
    }
}

pub trait ToSpan {
    fn to_span(&self) -> Span<'_>;
}

impl<T: Display> ToSpan for T {
    fn to_span(&self) -> Span<'_> {
        Span::raw(self.to_string())
    }
}

impl Display for Span<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.content.lines() {
            std::fmt::Display::fmt(line, f)?;
        }
        Ok(())
    }
}
