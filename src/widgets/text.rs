use std::{borrow::Cow, io::Write};

use unicode_width::UnicodeWidthStr;

use crate::{
    layout::Alignment,
    style::Style,
    widgets::{Line, Widget},
};

/// Text represents a collection of [Lines](Line)
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Text<'a> {
    /// The alignment of the lines in the widget
    pub alignment: Option<Alignment>,
    /// The style of the text widget
    pub style: Style,
    /// The lines composing the text widget
    pub lines: Vec<Line<'a>>,
}

impl<'a> Text<'a> {
    /// Create a new text widget with raw content.
    pub fn raw<T: Into<Cow<'a, str>>>(content: T) -> Self {
        let lines: Vec<_> = match content.into() {
            Cow::Borrowed("") => vec![Line::from("")],
            Cow::Borrowed(s) => s.lines().map(Line::from).collect(),
            Cow::Owned(s) if s.is_empty() => vec![Line::from("")],
            Cow::Owned(s) => s.lines().map(|l| Line::from(l.to_owned())).collect(),
        };

        Self::from(lines)
    }

    /// Create a new text widget with content and style.
    pub fn styled<T, S>(content: T, style: S) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Style>,
    {
        Self::raw(content).patch_style(style.into())
    }

    /// Get the unicode width of the text widget.
    pub fn width(&self) -> usize {
        UnicodeWidthStr::width(self)
    }

    /// Get the number of lines in the text widget.
    pub fn height(&self) -> usize {
        self.lines.len()
    }

    /// Patch the style of the text widget.
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn patch_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = self.style.patch(style.into());
        self
    }

    /// Set the style of the text widget.
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    /// Reset the style of the text widget.
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn reset_style(mut self) -> Self {
        self.style = Style::default();
        self
    }

    /// Set the alignment of the text widget.
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn alignment(self, alignment: Alignment) -> Self {
        Self {
            alignment: Some(alignment),
            ..self
        }
    }

    /// Returns an iterator over the lines of the text.
    pub fn iter(&self) -> core::slice::Iter<'_, Line<'a>> {
        self.lines.iter()
    }

    /// Returns an iterator that allows modifying each line.
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, Line<'a>> {
        self.lines.iter_mut()
    }
}

impl UnicodeWidthStr for Text<'_> {
    fn width(&self) -> usize {
        self.lines.iter().map(|l| l.width()).max().unwrap_or(0)
    }

    fn width_cjk(&self) -> usize {
        self.lines.iter().map(|l| l.width()).max().unwrap_or(0)
    }
}

impl<'a> From<&'a [Line<'a>]> for Text<'a> {
    fn from(value: &'a [Line]) -> Self {
        Self {
            lines: value.to_vec(),
            ..Default::default()
        }
    }
}

impl<'a> From<Vec<Line<'a>>> for Text<'a> {
    fn from(value: Vec<Line<'a>>) -> Self {
        Self {
            lines: value,
            ..Default::default()
        }
    }
}

impl Widget for Text<'_> {
    fn render<W: std::io::Write>(
        &self,
        width: &crate::layout::Width,
        terminal: &mut crate::terminal::Terminal<W>,
    ) {
        for line in &self.lines {
            terminal.render_widget(line, width);
            terminal.newline();
        }
    }
}
