use crate::{style::Style, widgets::Span};

/// A [Line] is a widget composed of a collection of [Spans](Span). This can be used to render a single
/// line of text with multiple styles.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Line<'a> {
    spans: Vec<Span<'a>>,
    style: Option<Style>,
}
