use crate::Style;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Cell {
    symbol: Option<String>,
    style: Style,
}

impl Cell {
    pub fn new<S: ToString, T: Into<Style>>(symbol: S, style: T) -> Self {
        Self {
            symbol: Some(symbol.to_string()),
            style: style.into(),
        }
    }

    pub fn empty() -> Self {
        Self {
            symbol: None,
            style: Style::default(),
        }
    }

    pub fn set_symbol<S: ToString>(&mut self, symbol: S) -> &mut Self {
        self.symbol = Some(symbol.to_string());
        self
    }

    pub fn set_style<S: Into<Style>>(&mut self, style: S) -> &mut Self {
        self.style = style.into();
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        self.symbol = None;
        self.style = Style::default();
        self
    }
}
