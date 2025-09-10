use crate::Style;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    symbol: Option<String>,
    style: Style,
}

impl Cell {
    pub fn empty() -> Self {
        Self {
            symbol: None,
            style: Style::default(),
        }
    }

    #[must_use]
    pub fn symbol(&self) -> &str {
        self.symbol.as_ref().map_or(" ", |s| s.as_str())
    }

    pub fn set_symbol(&mut self, symbol: &str) -> &mut Self {
        self.symbol = Some(String::from(symbol));
        self
    }

    pub fn set_char(&mut self, ch: char) -> &mut Self {
        let mut buf = [0; 4];
        self.symbol = Some(String::from(ch.encode_utf8(&mut buf)));
        self
    }

    pub fn set_style<S: Into<Style>>(&mut self, style: S) -> &mut Self {
        self.style = style.into();
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        self.style = Style::default();
        self.symbol = None;
        self
    }

    pub fn display(&self) -> String {
        self.style.apply(self.symbol()).to_string()
    }
}
