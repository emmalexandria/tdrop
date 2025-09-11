use crate::style::Style;

/// Cell is a representation of a single terminal character.
///
/// It is unicode/width aware, meaning that one cell does not necessarily equal one terminal width.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Cell {
    style: Style,
    symbol: Option<String>,
}
