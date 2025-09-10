use std::borrow::Cow;

use crate::{buffer::buffer::Buffer, layout::rect::Rect, widgets::Widget, Style};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Span<'a> {
    content: Cow<'a, str>,
    style: Style,
}

impl<'a> Span<'a> {
    #[must_use]
    pub fn raw<T: Into<Cow<'a, str>>>(content: T) -> Self {
        Self {
            content: content.into(),
            style: Style::default(),
        }
    }

    #[must_use]
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

    #[must_use]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    #[must_use]
    pub fn content<T>(mut self, content: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.content = content.into();
        self
    }
}

impl Widget for Span<'_> {
    fn render(self, frame: Rect) -> String {
        let mut buf = Buffer::empty(frame);
        buf.set_string(0, 0, self.content, self.style);

        buf.render()
    }
}
