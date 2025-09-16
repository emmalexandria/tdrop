use std::cmp::{max, min};

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

    #[must_use = "method returns the modified value"]
    pub fn intersection(self, other: Self) -> Self {
        let x1 = max(self.x, other.x);
        let y1 = max(self.y, other.y);
        let x2 = min(self.right(), other.right());
        let y2 = min(self.bottom(), other.bottom());
        Self {
            x: x1,
            y: y1,
            width: x2.saturating_sub(x1),
            height: y2.saturating_sub(y1),
        }
    }

    /// Returns true if the two `Rect`s intersect.
    pub const fn intersects(self, other: Self) -> bool {
        self.x < other.right()
            && self.right() > other.x
            && self.y < other.bottom()
            && self.bottom() > other.y
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
