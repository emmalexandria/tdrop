use std::{borrow::Cow, fmt::Display, io::Write};

use unicode_truncate::UnicodeTruncateStr;
use unicode_width::UnicodeWidthStr;

use crate::{
    layout::{Alignment, Width},
    style::{AsStyle, AsStyleMut, Style, Stylize},
    terminal::Terminal,
    widgets::{Span, Widget},
};

/// A [Line] is a widget composed of a collection of [Spans](Span). This can be used to render a single
/// line of text with multiple styles.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Line<'a> {
    /// The style of the line, which will be patched to the style of the spans
    pub style: Style,

    /// The alignment of the line of text
    pub alignment: Option<Alignment>,

    /// The [Spans](Span) composing this line.
    pub spans: Vec<Span<'a>>,
}

fn cow_to_spans<'a>(content: impl Into<Cow<'a, str>>) -> Vec<Span<'a>> {
    match content.into() {
        Cow::Borrowed(s) => s.lines().map(Span::raw).collect(),
        Cow::Owned(s) => s.lines().map(|v| Span::raw(v.to_string())).collect(),
    }
}

impl<'a> Line<'a> {
    /// Create a new line from raw content without a style
    pub fn raw<T>(content: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self {
            spans: cow_to_spans(content),
            ..Default::default()
        }
    }

    /// Create a new line with the given content and style
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

    /// Set the [Spans](Span) of the line
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn spans<I>(mut self, spans: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Span<'a>>,
    {
        self.spans = spans.into_iter().map(Into::into).collect();
        self
    }

    /// Set the [Style] of this line of text
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    /// Set the [Alignment] of this line of text
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn alignment(self, alignment: Alignment) -> Self {
        Self {
            alignment: Some(alignment),
            ..self
        }
    }

    /// Left aligns the line of text
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn left_aligned(self) -> Self {
        self.alignment(Alignment::Left)
    }

    /// Centers the line of text
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn centered(self) -> Self {
        self.alignment(Alignment::Center)
    }

    /// Right aligns the line of text
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn right_aligned(self) -> Self {
        self.alignment(Alignment::Right)
    }

    /// Get the unicode width of the line
    pub fn width(&self) -> usize {
        UnicodeWidthStr::width(self)
    }

    /// Patches the [Style] of the line
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn patch_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = self.style.patch(style.into());
        self
    }

    /// Return the [Spans](Span) of the line as an iterator
    pub fn iter(&self) -> std::slice::Iter<'_, Span<'a>> {
        self.spans.iter()
    }

    /// Return the [Spans](Span) of the line as a mutable iterator
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Span<'a>> {
        self.spans.iter_mut()
    }

    /// Push a [Span] to the end of the line
    pub fn push_span<T: Into<Span<'a>>>(&mut self, span: T) {
        self.spans.push(span.into());
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

impl AsStyle for Line<'_> {
    fn as_style(&self) -> &Style {
        &self.style
    }
}

impl AsStyleMut for Line<'_> {
    fn as_style_mut(&mut self) -> &mut Style {
        &mut self.style
    }
}

impl<'a> Stylize for Line<'a> {
    type Styled = Self;

    fn stylize(self) -> Self::Styled {
        self
    }
}

impl<'a> From<String> for Line<'a> {
    fn from(value: String) -> Self {
        Self {
            spans: vec![Span::raw(value)],
            ..Default::default()
        }
    }
}

impl<'a> From<&str> for Line<'a> {
    fn from(value: &str) -> Self {
        Self {
            spans: vec![Span::raw(value.to_string())],
            ..Default::default()
        }
    }
}

impl<'a> IntoIterator for Line<'a> {
    type Item = Span<'a>;
    type IntoIter = std::vec::IntoIter<Span<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.spans.into_iter()
    }
}

impl<'a> IntoIterator for &'a Line<'a> {
    type Item = &'a Span<'a>;
    type IntoIter = std::slice::Iter<'a, Span<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Line<'a> {
    type Item = &'a mut Span<'a>;
    type IntoIter = std::slice::IterMut<'a, Span<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl Widget for Line<'_> {
    fn render<W: std::io::Write>(&self, width: &crate::layout::Width, terminal: &mut Terminal<W>) {
        Widget::render(&self, width, terminal)
    }
}

impl Widget for &Line<'_> {
    fn render<W: std::io::Write>(&self, width: &Width, terminal: &mut Terminal<W>) {
        self.render_with_alignment(width, terminal, None);
    }
}

impl Display for Line<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for span in &self.spans {
            write!(f, "{span}")?;
        }
        Ok(())
    }
}

impl Line<'_> {
    pub(crate) fn render_with_alignment<W: Write>(
        &self,
        width: &Width,
        terminal: &mut Terminal<W>,
        parent_alignment: Option<Alignment>,
    ) {
        let intersection = width.intersection(&terminal.width);
        let width = intersection.width as usize;
        if width == 0 {
            return;
        }

        let line_width = self.width();
        if line_width == 0 {
            return;
        }

        let alignment = self.alignment.or(parent_alignment);
        let can_render_complete = line_width <= intersection.width as usize;
        if can_render_complete {
            let indent_width = match alignment {
                Some(Alignment::Center) => width.saturating_sub(line_width) / 2,
                Some(Alignment::Right) => width.saturating_sub(line_width),
                Some(Alignment::Left) | None => 0,
            };
            let indent_width = u16::try_from(indent_width).unwrap_or(u16::MAX);

            let width = intersection.indent_x(indent_width);
            println!("{:?}, {:?}", intersection, width);
            render_spans(&self.spans, width, terminal, 0);
        } else {
            let skip_width = match alignment {
                Some(Alignment::Center) => (line_width.saturating_sub(line_width)) / 2,
                Some(Alignment::Right) => line_width.saturating_sub(line_width),
                Some(Alignment::Left) | None => 0,
            };
            render_spans(&self.spans, intersection, terminal, skip_width);
        }
    }
}

fn render_spans<W: Write>(
    spans: &[Span],
    mut width: Width,
    terminal: &mut Terminal<W>,
    span_skip_width: usize,
) {
    for (span, span_width, offset) in spans_after_width(spans, span_skip_width) {
        width = width.indent_x(offset);

        if width.width == 0 {
            break;
        }

        span.render(&width, terminal);
        let span_width = u16::try_from(span_width).unwrap_or(u16::MAX);
        width = width.indent_x(span_width);
    }
}

fn spans_after_width<'a>(
    spans: &'a [Span],
    mut skip_width: usize,
) -> impl Iterator<Item = (Span<'a>, usize, u16)> {
    spans
        // Iterate through each span
        .iter()
        // Map them into the span and the span's unicode width
        .map(|s| (s, s.width()))
        .filter_map(move |(span, span_width)| {
            // Remove span if it is less wide than the skip width
            if skip_width >= span_width {
                // Reduce the skip width for the next iteration
                skip_width = skip_width.saturating_sub(span_width);
                return None;
            }

            // Otherwise, calculate the available width
            let available_width = span_width.saturating_sub(skip_width);
            // Set the skip width oo zero
            skip_width = 0;
            // Return the span, span width, and the width available to the span
            Some((span, span_width, available_width))
        })
        .map(|(span, span_width, available_width)| {
            if span_width < available_width {
                return (span.clone(), span_width, 0u16);
            }

            let (content, actual_width) = span.content.unicode_truncate_start(available_width);

            let first_grapheme_offset = available_width.saturating_sub(actual_width);
            let first_grapheme_offset = u16::try_from(first_grapheme_offset).unwrap_or(u16::MAX);
            (
                Span::styled(content, span.style),
                actual_width,
                first_grapheme_offset,
            )
        })
}
