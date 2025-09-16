use crate::style::{Attributes, Color};

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
}
