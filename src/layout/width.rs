use std::io::Write;

use crate::terminal::{Terminal, TerminalInput};

/// [Width] represents a width within the terminal. Widgets require a [Width] in order to determine
/// their maximum output length and align content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Width {
    /// The horizontal offset of the [Width]
    pub x: u16,
    /// The width of the [Width]
    pub width: u16,
}

impl Default for Width {
    fn default() -> Self {
        Self {
            x: 0,
            width: crossterm::terminal::size().ok().map(|w| w.0).unwrap_or(80),
        }
    }
}

impl Width {
    /// A [Width] of zero
    pub const ZERO: Self = Self::new(0, 0);

    /// Create a new [Width]
    pub const fn new(x: u16, width: u16) -> Self {
        Self { x, width }
    }

    /// Returns the right of the width (first cell outside the width)
    pub fn right(self) -> u16 {
        self.x.saturating_add(self.width)
    }

    /// Returns the left of a width (first cell of the width)
    pub fn left(self) -> u16 {
        self.x
    }

    /// Checks whether this [Width] contains another [Width]
    pub fn contains(&self, other: &Self) -> bool {
        let start_intersect = self.intersects(other);
        start_intersect && other.width < self.width - (other.x - self.x)
    }

    /// Check whether this [Width] intersects with another [Width]
    pub fn intersects(&self, other: &Self) -> bool {
        return other.x >= self.x && other.x < self.x + self.width;
    }

    /// Returns the inersection between this [Width] and another [Width] as a [Width].
    pub fn intersection(&self, other: &Self) -> Self {
        if !self.intersects(other) {
            return Self::ZERO;
        }

        let intersection_width = other.width.min(self.width - other.x);

        Self::new(other.x, intersection_width)
    }

    /// Write to the width on the given terminal
    pub fn write<W: Write, I: TerminalInput>(&self, text: I, term: &mut Terminal<W>) -> u16 {
        let intersection = term.width.intersection(self);

        let _ = term.move_to(intersection.x);
        let _ = term.printn(text, intersection.width as usize, true);

        intersection.width
    }

    /// Write to the width on the given terminal and insert a new line
    pub fn writeln<W: Write, I: TerminalInput>(&self, text: I, term: &mut Terminal<W>) -> u16 {
        let ret = self.write(text, term);
        let _ = term.newline();

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Width;

    #[test]
    fn test_contains() {
        let parent = Width::new(20, 30);
        let child = Width::new(25, 4);

        assert!(parent.contains(&child))
    }

    #[test]
    fn test_intersects() {
        assert!(Width::new(5, 25).intersects(&Width::new(6, 39)))
    }

    #[test]
    fn test_intersection() {
        let parent = Width::new(0, 50);
        let child = Width::new(5, 100);
        let expected = Width::new(5, 45);

        assert_eq!(parent.intersection(&child), expected)
    }
}
