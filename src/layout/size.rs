use std::fmt::Display;

use crate::layout::rect::Rect;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    pub const ZERO: Self = Self::new(0, 0);

    pub const fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}

impl From<(u16, u16)> for Size {
    fn from(value: (u16, u16)) -> Self {
        Self {
            width: value.0,
            height: value.1,
        }
    }
}

impl From<Rect> for Size {
    fn from(value: Rect) -> Self {
        Self {
            width: value.width,
            height: value.height,
        }
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

#[cfg(test)]
mod tests {
    use crate::layout::size::Size;

    #[test]
    fn new() {
        let size = Size::new(10, 20);
        assert_eq!(size.width, 10);
        assert_eq!(size.height, 20);
    }

    #[test]
    fn from_tuple() {
        let size: Size = (10, 20).into();
        assert_eq!(size.width, 10);
        assert_eq!(size.height, 20);
    }

    #[test]
    fn display() {
        let size = Size::new(35, 50);

        assert_eq!(size.to_string(), "35x50");
    }
}
