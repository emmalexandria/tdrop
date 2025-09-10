use crate::layout::position::Position;

pub struct Coordinate(u16, u16);

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Rect {
    pub x: u16,
    pub y: u16,

    pub width: u16,
    pub height: u16,
}

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Offset {
    pub x: i32,
    pub y: i32,
}

impl Rect {
    pub const ZERO: Self = Self {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };

    pub const fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        let max_width = u16::MAX - x;
        let max_height = u16::MAX - y;
        let width = if width > max_width { max_width } else { width };
        let height = if height > max_height {
            max_height
        } else {
            height
        };

        Self {
            x,
            y,
            width,
            height,
        }
    }

    /** This function returns a rect which is the width of the terminal with an unbounded height.
     * If the terminal width cannot be retrieved, returns one with width 80 */
    pub fn term_rect() -> Self {
        let size = crossterm::terminal::size().unwrap_or((80, 0));
        Self {
            x: 0,
            y: 0,
            width: size.0,
            height: 0,
        }
    }

    pub const fn area(self) -> u32 {
        (self.width as u32) * (self.height as u32)
    }

    pub const fn is_empty(self) -> bool {
        self.width == 0
    }

    pub const fn left(self) -> u16 {
        self.x
    }

    pub const fn right(self) -> u16 {
        self.x.saturating_add(self.width)
    }

    pub const fn top(self) -> u16 {
        self.y
    }

    pub const fn bottom(self) -> u16 {
        self.y.saturating_add(self.height)
    }

    pub const fn contains(self, position: Position) -> bool {
        position.x >= self.x
            && position.x < self.right()
            && position.y >= self.y
            && position.y < self.bottom()
    }

    pub fn as_position(self) -> Position {
        Position {
            x: self.x,
            y: self.y,
        }
    }
}
