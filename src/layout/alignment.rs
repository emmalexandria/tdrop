/// The horizontal alignment of text
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Alignment {
    /// Left alignment
    #[default]
    Left,
    /// Center alignment
    Center,
    /// Right alignment
    Right,
}
