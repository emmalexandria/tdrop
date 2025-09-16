#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl From<(u16, u16)> for Position {
    fn from(value: (u16, u16)) -> Self {
        Position {
            x: value.0,
            y: value.1,
        }
    }
}

impl Into<(u16, u16)> for Position {
    fn into(self) -> (u16, u16) {
        (self.x, self.y)
    }
}
