use crate::Style;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct StyledGrapheme<'a> {
    pub symbol: &'a str,
    pub style: Style,
}

impl<'a> StyledGrapheme<'a> {
    pub fn new<S: Into<Style>>(symbol: &'a str, style: S) -> Self {
        Self {
            symbol,
            style: style.into(),
        }
    }
}
