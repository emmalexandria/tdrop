use crate::style::{Attributes, Color, Style};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cell {
    symbol: Option<String>,

    pub fg: Color,
    pub bg: Color,

    pub attributes: Attributes,

    pub skip: bool,
}

impl Cell {
    pub const EMPTY: Self = Self::new(None, Color::Reset, Color::Reset, Attributes::none());

    pub const fn new(symbol: Option<String>, fg: Color, bg: Color, attributes: Attributes) -> Self {
        Self {
            symbol,
            fg,
            bg,
            attributes,
            skip: false,
        }
    }

    pub fn symbol(&self) -> &str {
        self.symbol.as_ref().map_or(" ", |s| s.as_str())
    }

    pub fn set_symbol(&mut self, symbol: &str) -> &mut Self {
        self.symbol = Some(String::from(symbol));
        self
    }

    pub fn reset(&mut self) {
        *self = Self::EMPTY
    }

    pub fn set_style<S: Into<Style>>(&mut self, style: S) -> &mut Self {
        let style = style.into();
        if let Some(c) = style.fg {
            self.fg = c;
        }
        if let Some(c) = style.bg {
            self.bg = c;
        }

        self.attributes = style.attributes;
        self
    }
}
