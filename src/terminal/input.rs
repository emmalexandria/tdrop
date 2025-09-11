use crate::style::Style;

/// A trait for types which provde input to the terminal
pub trait TerminalInput {
    /// Return the content of the input
    fn content(&self) -> String;

    /// Return the style of the input
    fn style(&self) -> Style {
        Style::default()
    }
}

impl TerminalInput for String {
    fn content(&self) -> String {
        self.clone()
    }
}

impl TerminalInput for &str {
    fn content(&self) -> String {
        self.to_string()
    }
}

impl TerminalInput for &String {
    fn content(&self) -> String {
        self.to_string()
    }
}

impl TerminalInput for char {
    fn content(&self) -> String {
        self.to_string()
    }
}
