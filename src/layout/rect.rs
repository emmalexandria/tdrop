#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Rect {
    /** Width represents the width of the area */
    pub width: u16,
    /** Height represents the optional height of the buffer, defaults to none */
    pub height: Option<u16>,
    /** Represents the x position relative to the current line */
    pub x: u16,
    /** Represents the y position relative to the current line */
    pub y: u16,
    pub expand: bool,
}

impl Rect {
    pub fn new(x: u16, y: u16, width: u16, height: Option<u16>) -> Self {
        let expand = if height.is_some() { false } else { true };
        Self {
            x,
            y,
            width,
            height,
            expand,
        }
    }

    pub fn expands(mut self, expand: bool) -> Self {
        self.expand = expand;
        self
    }

    pub fn top(self) -> u16 {
        return self.y;
    }

    pub fn left(self) -> u16 {
        return self.x;
    }

    pub fn bottom(self) -> Option<u16> {
        if let Some(h) = self.height {
            return Some(self.y.overflowing_add(h).0);
        }

        None
    }

    pub fn right(self) -> u16 {
        self.x.overflowing_add(self.width).0
    }

    pub fn area(self) -> u32 {
        (self.width as u32 * self.height.unwrap_or(1) as u32)
    }
}
