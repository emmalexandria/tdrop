use crossterm::style::{Attributes, Color, Stylize};

use crate::Style;

#[derive(Debug, Clone)]
pub struct Cell {
    symbol: Option<String>,

    pub style: Style,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.style.apply(self.symbol()))
    }
}

impl Cell {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: Some(symbol.to_string()),
            ..Self::empty()
        }
    }

    pub fn empty() -> Self {
        Self {
            symbol: None,
            style: Style::default(),
        }
    }

    pub fn symbol(&self) -> &str {
        self.symbol.as_ref().map_or(" ", |s| s.as_str())
    }

    pub fn set_symbol(&mut self, symbol: &str) -> &mut Self {
        self.symbol = Some(symbol.to_string());
        self
    }

    pub fn set_style<S: Into<Style>>(&mut self, style: S) -> &mut Self {
        self.style = style.into();
        self
    }

    pub fn reset(&mut self) {
        *self = Self::empty()
    }

    pub fn style(&self) -> Style {
        self.style
    }
}
