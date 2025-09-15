#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Width {
    pub offset: u16,
    pub width: u16,
}

impl Width {
    /// A width with zero width and zero offset.
    pub const ZERO: Self = Self::new(0, 0);

    /// Create a new width with the given x offset and width.
    pub const fn new(offset: u16, width: u16) -> Self {
        Self { offset, width }
    }

    /// Set the size of the width
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    #[must_use = "moves the value of self and returns the modified value"]
    pub fn offset(mut self, offset: u16) -> Self {
        self.offset = offset;
        self
    }

    /// Returns the right of the width (first cell outside the width)
    pub fn right(self) -> u16 {
        self.offset.saturating_add(self.width)
    }

    /// Returns the left of a width (first cell of the width)
    pub fn left(self) -> u16 {
        self.offset
    }

    /// Checks whether this [Width] contains another [Width]
    pub fn contains(&self, other: &Self) -> bool {
        let start_intersect = self.intersects(other);
        start_intersect && other.width < self.width - (other.offset - self.offset)
    }

    /// Check whether this [Width] intersects with another [Width]
    pub fn intersects(&self, other: &Self) -> bool {
        return other.offset >= self.offset && other.offset < self.offset + self.width;
    }

    /// Returns the inersection between this [Width] and another [Width] as a [Width].
    pub fn intersection(&self, other: &Self) -> Self {
        if !self.intersects(other) {
            return Self::ZERO;
        }

        let intersection_width = other.width.min(self.width - other.offset);

        Self::new(other.offset, intersection_width)
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
