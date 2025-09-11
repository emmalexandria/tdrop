use crate::style::Style;

/// A trait for types which provde input to the terminal
pub trait TerminalInput {
    /// Return the content of the input
    fn content(&self) -> String;

    /// Return the style of the input
    fn style(&self) -> Style;
}

impl<S: AsRef<str>> TerminalInput for S {
    fn content(&self) -> String {
        self.as_ref().to_string()
    }

    fn style(&self) -> Style {
        Style::default()
    }
}
