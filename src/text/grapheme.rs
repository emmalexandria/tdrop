use crate::Style;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct StyledGrapheme<'a> {
    pub symbol: &'a str,
    pub style: Style,
}
