use std::{borrow::Cow, fmt};

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::{buffer::Buffer, layout, text::grapheme::StyledGrapheme, widgets::Widget, Style};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
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

    pub fn width(&self) -> usize {
        UnicodeWidthStr::width(self)
    }

    pub fn styled_graphemes<S: Into<Style>>(
        &'a self,
        base_style: S,
    ) -> impl Iterator<Item = StyledGrapheme<'a>> {
        self.content
            .as_ref()
            .graphemes(true)
            .filter(|g| !g.contains(char::is_control))
            .map(move |g| StyledGrapheme {
                symbol: g,
                style: self.style,
            })
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

impl fmt::Display for Span<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.content.lines() {
            fmt::Display::fmt(line, f)?;
        }
        Ok(())
    }
}

impl Widget for Span<'_> {
    fn render(&self, rect: &layout::rect::Rect) -> Buffer {
        let mut buffer = Buffer::new(rect);
        buffer.set_string(0, 0, self.content.as_ref(), self.style);
        buffer
    }
}
