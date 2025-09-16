use crate::layout::{Position, Size};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Rect {
    pub const fn area(self) -> u32 {
        (self.width as u32) * (self.height as u32)
    }

    pub const fn right(self) -> u16 {
        self.x.saturating_add(self.width)
    }

    pub const fn left(self) -> u16 {
        self.x
    }

    pub const fn top(self) -> u16 {
        self.y
    }

    pub const fn bottom(self) -> u16 {
        self.y.saturating_add(self.height)
    }

    pub const fn as_size(self) -> Size {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    pub const fn as_position(self) -> Position {
        Position {
            x: self.x,
            y: self.y,
        }
    }

    pub const fn contains(self, position: Position) -> bool {
        position.x >= self.x
            && position.x <= self.right()
            && position.y >= self.y
            && position.y <= self.bottom()
    }
}

impl From<Size> for Rect {
    fn from(size: Size) -> Self {
        Self {
            x: 0,
            y: 0,
            width: size.width,
            height: size.height,
        }
    }
}
