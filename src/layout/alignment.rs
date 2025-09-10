#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HorizontalAlignment {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerticalAlignment {
    #[default]
    Top,
    Middle,
    Bottom,
}
