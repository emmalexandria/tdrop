#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: Option<u16>,
}

impl Rect {
    pub const fn new(x: u16, y: u16, width: u16, height: Option<u16>) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn area(self) -> u32 {
        return (self.width as u32 * self.height.unwrap_or(1) as u32);
    }

    pub fn right(self) -> u16 {
        self.x + self.width
    }

    /// Returns a new [[Rect]] at (0,0) sized to the terminal's width with an unbounded height.
    pub fn term() -> Option<Self> {
        let size = crossterm::terminal::size();

        if let Ok((width, _)) = size {
            return Some(Self {
                x: 0,
                y: 0,
                width,
                height: None,
            });
        }

        None
    }
}
