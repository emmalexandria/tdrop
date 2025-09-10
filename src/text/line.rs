use std::borrow::Cow;
use std::vec;

use unicode_width::UnicodeWidthStr;

use crate::buffer::Buffer;
use crate::layout::alignment::HorizontalAlignment;
use crate::layout::rect::Rect;
use crate::text::grapheme::StyledGrapheme;
use crate::text::span::Span;
use crate::widgets::Widget;
use crate::Style;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Line<'a> {
    pub style: Style,

    pub alignment: Option<HorizontalAlignment>,
    pub spans: Vec<Span<'a>>,
}

fn cow_to_spans<'a>(content: impl Into<Cow<'a, str>>) -> Vec<Span<'a>> {
    match content.into() {
        Cow::Borrowed(s) => s.lines().map(Span::raw).collect(),
        Cow::Owned(s) => s.lines().map(|v| Span::raw(v.to_string())).collect(),
    }
}

impl<'a> Line<'a> {
    pub fn raw<T>(content: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self {
            spans: cow_to_spans(content),
            ..Default::default()
        }
    }

    pub fn styled<T, S>(content: T, style: S) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Style>,
    {
        Self {
            spans: cow_to_spans(content),
            style: style.into(),
            ..Default::default()
        }
    }

    pub fn spans<I>(mut self, spans: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Span<'a>>,
    {
        self.spans = spans.into_iter().map(Into::into).collect();
        self
    }

    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    pub fn alignment(self, alignment: HorizontalAlignment) -> Self {
        Self {
            alignment: Some(alignment),
            ..self
        }
    }

    pub fn left_aligned(self) -> Self {
        self.alignment(HorizontalAlignment::Left)
    }

    pub fn centered(self) -> Self {
        self.alignment(HorizontalAlignment::Center)
    }

    pub fn right_aligned(self) -> Self {
        self.alignment(HorizontalAlignment::Right)
    }

    pub fn width(&self) -> usize {
        UnicodeWidthStr::width(self)
    }

    pub fn styled_graphemes<S: Into<Style>>(
        &'a self,
        base_style: S,
    ) -> impl Iterator<Item = StyledGrapheme<'a>> {
        self.spans
            .iter()
            .flat_map(move |span| span.styled_graphemes(self.style))
    }
}

impl UnicodeWidthStr for Line<'_> {
    fn width(&self) -> usize {
        self.spans.iter().map(UnicodeWidthStr::width).sum()
    }

    fn width_cjk(&self) -> usize {
        self.spans.iter().map(UnicodeWidthStr::width_cjk).sum()
    }
}

impl<'a> IntoIterator for Line<'a> {
    type Item = Span<'a>;
    type IntoIter = vec::IntoIter<Span<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.spans.into_iter()
    }
}

impl<'a> IntoIterator for &'a Line<'a> {
    type Item = &'a Span<'a>;
    type IntoIter = core::slice::Iter<'a, Span<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Line<'a> {
    type Item = &'a mut Span<'a>;
    type IntoIter = core::slice::IterMut<'a, Span<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl From<String> for Line<'_> {
    fn from(s: String) -> Self {
        Self::raw(s)
    }
}

impl<'a> From<&'a str> for Line<'a> {
    fn from(s: &'a str) -> Self {
        Self::raw(s)
    }
}

impl<'a> From<Cow<'a, str>> for Line<'a> {
    fn from(s: Cow<'a, str>) -> Self {
        Self::raw(s)
    }
}

impl<'a> From<Vec<Span<'a>>> for Line<'a> {
    fn from(spans: Vec<Span<'a>>) -> Self {
        Self {
            spans,
            ..Default::default()
        }
    }
}

impl<'a> From<Span<'a>> for Line<'a> {
    fn from(span: Span<'a>) -> Self {
        Self::from(vec![span])
    }
}

impl<'a> From<Line<'a>> for String {
    fn from(line: Line<'a>) -> Self {
        line.iter().fold(Self::new(), |mut acc, s| {
            acc.push_str(s.content.as_ref());
            acc
        })
    }
}

impl<'a, T> FromIterator<T> for Line<'a>
where
    T: Into<Span<'a>>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::from(iter.into_iter().map(Into::into).collect::<Vec<_>>())
    }
}

/// Adds a `Span` to a `Line`, returning a new `Line` with the `Span` added.
impl<'a> core::ops::Add<Span<'a>> for Line<'a> {
    type Output = Self;

    fn add(mut self, rhs: Span<'a>) -> Self::Output {
        self.spans.push(rhs);
        self
    }
}

/// Adds two `Line`s together, returning a new `Text` with the contents of the two `Line`s.
impl<'a> core::ops::AddAssign<Span<'a>> for Line<'a> {
    fn add_assign(&mut self, rhs: Span<'a>) {
        self.spans.push(rhs);
    }
}

impl<'a> Extend<Span<'a>> for Line<'a> {
    fn extend<T: IntoIterator<Item = Span<'a>>>(&mut self, iter: T) {
        self.spans.extend(iter);
    }
}

impl Widget for Line<'_> {
    fn render(self, area: &Rect) -> Buffer {
        Widget::render(&self, area)
    }
}

impl Line<'_> {
    /// An internal implementation method for `Widget::render` that allows the parent widget to
    /// define a default alignment, to be used if `Line::alignment` is `None`.
    pub(crate) fn render_with_alignment(
        &self,
        area: Rect,
        parent_alignment: Option<HorizontalAlignment>,
    ) -> Buffer {
        let buffer = Buffer::new(&area);
        let area = Rect { height: 1, ..area };
        let line_width = self.width();
        if line_width == 0 {
            return buffer;
        }

        buf.set_style(area, self.style);

        let alignment = self.alignment.or(parent_alignment);

        let area_width = usize::from(area.width);
        let can_render_complete_line = line_width <= area_width;
        if can_render_complete_line {
            let indent_width = match alignment {
                Some(Alignment::Center) => (area_width.saturating_sub(line_width)) / 2,
                Some(Alignment::Right) => area_width.saturating_sub(line_width),
                Some(Alignment::Left) | None => 0,
            };
            let indent_width = u16::try_from(indent_width).unwrap_or(u16::MAX);
            let area = area.indent_x(indent_width);
            render_spans(&self.spans, area, buf, 0);
        } else {
            // There is not enough space to render the whole line. As the right side is truncated by
            // the area width, only truncate the left.
            let skip_width = match alignment {
                Some(Alignment::Center) => (line_width.saturating_sub(area_width)) / 2,
                Some(Alignment::Right) => line_width.saturating_sub(area_width),
                Some(Alignment::Left) | None => 0,
            };
            render_spans(&self.spans, area, buf, skip_width);
        }
    }
}
