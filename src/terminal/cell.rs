use crate::style::{Attributes, Color};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cell {
    symbol: Option<String>,

    fg: Color,
    bg: Color,

    attributes: Attributes,
}
