use std::borrow::Cow;

use crate::{
    buffer::buffer::Buffer,
    layout::{alignment::HorizontalAlignment, rect::Rect},
    text::span::Span,
    widgets::Widget,
    Style,
};

pub struct Line<'a> {
    style: Style,

    alignment: HorizontalAlignment,

    spans: Vec<Span<'a>>,
}

impl<'a> Line<'a> {
    pub fn raw<T>(content: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self {
            style: Style::default(),
            alignment: HorizontalAlignment::Left,
            spans: cow_to_spans(content.into()),
        }
    }

    pub fn styled<T, S>(content: T, style: S) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Style>,
    {
        Self {
            style: style.into(),
            alignment: HorizontalAlignment::Left,
            spans: cow_to_spans(content.into()),
        }
    }

    pub fn push<T, S>(&mut self, content: T, style: S) -> &mut Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Style>,
    {
        self.spans.push(Span::styled(content, style));
        self
    }
}

fn cow_to_spans<'a>(content: Cow<'a, str>) -> Vec<Span<'a>> {
    vec![Span::raw(content)]
}

impl Widget for Line<'_> {
    fn render(self, frame: Rect) -> String {
        let buf = Buffer::empty(frame);

        buf.render()
    }
}
