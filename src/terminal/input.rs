use std::fmt::Display;

use crate::style::{Style, StyledString};

pub trait Input {
    fn content(&self) -> String;
    fn style(&self) -> Style {
        Style::default()
    }
}

impl Input for String {
    fn content(&self) -> String {
        self.clone()
    }
}

impl Input for &str {
    fn content(&self) -> String {
        self.to_string()
    }
}

impl Input for &String {
    fn content(&self) -> String {
        self.to_string()
    }
}

impl<D: Display> Input for StyledString<D> {
    fn content(&self) -> String {
        self.content().to_string()
    }

    fn style(&self) -> Style {
        *self.style()
    }
}
