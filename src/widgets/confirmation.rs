//! This module provides a basic confirmation prompt implemented as a [StatefulWidget].
//!
//! Please note that like all [StatefulWidget]s provided by `tdrop`, responsibility for capturing
//! user input and updating the widget state is placed on your application.

use std::{borrow::Cow, io::Write};

use crate::{
    layout::Width,
    style::{Attribute, Color, Style, Stylize},
    terminal::Terminal,
    theme::Theme,
    widgets::{Line, Span, StatefulWidget, Widget},
};

/// A struct representing the state of the confirmation widget
pub struct ConfirmationState {
    /// Whether yes or no is selected
    pub selected: bool,
    /// Whether the confirmation state is done (e.g. result has been selected)
    pub done: bool,
}

/// A simple yes/no confirmation [StatefulWidget].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Confirmation<'a> {
    theme: Theme,
    question: Cow<'a, str>,
}

impl<'a> Confirmation<'a> {
    /// Create a new confirmation widget with the given question.
    pub fn new<S>(question: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        let theme = Theme::default();
        Self {
            theme,
            question: question.into(),
        }
    }

    fn get_line(&self, state: &ConfirmationState) -> Line<'a> {
        let mut line = Line::default().spans([
            Span::raw(self.question.clone()),
            Span::raw(" "),
            Span::raw("[Y]es"),
            Span::raw("/"),
            Span::raw("[N]o"),
        ]);

        let mut spans: Vec<&mut Span<'a>> = line.iter_mut().collect();
        if state.selected {
            spans[2].set_style(Style::new().bg(self.theme.success));
            spans[4].set_style(Style::new().fg(self.theme.error));
        } else {
            spans[2].set_style(Style::new().fg(self.theme.success));
            spans[4].set_style(Style::new().bg(self.theme.error));
        }

        line
    }

    /// Set the theme of the confirmation widget.
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}

impl StatefulWidget for Confirmation<'_> {
    type State = ConfirmationState;

    fn render<W: std::io::Write>(
        &self,
        width: &crate::layout::Width,
        terminal: &mut crate::terminal::Terminal<W>,
        state: &ConfirmationState,
    ) -> bool {
        if state.done {
            self.render_completed(width, terminal, state);
            return false;
        }
        let line = self.get_line(state);
        terminal.render_widget(&line, width);
        true
    }
}

impl Confirmation<'_> {
    fn render_completed<W: Write>(
        &self,
        width: &Width,
        terminal: &mut Terminal<W>,
        state: &ConfirmationState,
    ) {
        terminal.clear_n(1);
        let mut success_text = match state.selected {
            true => Line::default().spans([Span::raw("✓")
                .with(self.theme.success)
                .attribute(Attribute::Bold)]),
            false => Line::default().spans([Span::raw("x")
                .with(self.theme.error)
                .attribute(Attribute::Bold)]),
        };
        success_text.push_span(self.question.clone());

        terminal.render_widget(success_text, width);
        terminal.newline();
    }
}
