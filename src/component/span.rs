use std::borrow::Cow;

use crate::{component::Component, layout::Width, style::Style};

pub struct Span<'a> {
    pub content: Cow<'a, str>,
    pub style: Style,
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
}

impl Component for Span<'_> {
    fn render<W: std::io::Write>(
        self,
        width: crate::layout::Width,
        term: &mut crate::terminal::Terminal<W>,
    ) {
        let intersection = Width::from(term.width()).intersection(&width);

        term.print(self.style.apply(self.content))
    }
}
