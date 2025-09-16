//! Implements attributes like `Bold`, `Italic`, etc.
//!
//! This module is used in conjunction with the [color](crate::style::color) module to implement
//! terminal styling. Much like the rest of the [style](super) module, it currently converts to the
//! crossterm types for rendering.

use crossterm::style;

/// A trait which is used in the conversion from our attributes type to the crossterm type
trait GetAttributes<T> {
    fn get_attributes(self) -> Vec<T>;
}

impl GetAttributes<style::Attribute> for style::Attributes {
    fn get_attributes(self) -> Vec<style::Attribute> {
        style::Attribute::iterator()
            .filter(|a| self.has(*a))
            .collect()
    }
}

/// [Attribute] is an enum of individual terminal attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Attribute {
    // Basic attributes
    /// Resets all attributes
    Reset = 0,
    /// Increases text intensity
    Bold = 1,
    /// Decreases the text intensity
    Dim = 2,
    /// Emphasizes the text
    Italic = 3,
    /// Underlines the text
    Underlined = 4,
    /// Makes the text blink slowly
    SlowBlink = 5,
    /// Makes the text blink rapidly
    RapidBlink = 6,
    /// Reverses the background and foreground
    Reverse = 7,
    /// Hides the text
    Hidden = 8,
    /// Crosses out the text
    Strikethrough = 9,
    /// Turns off the `Bold` attribute. Prefer to use `Normalintensity`
    NoBold = 21,
    /// Switches text back to normal intensity (`Bold`, `Italic`)
    NormalIntensity = 22,
    /// Turns off the `Italic` attribute
    NoItalic = 23,
    /// Turns off the `Underlined` attribute
    NoUnderline = 24,
    /// Turns off the blink attributes (`SlowBlink` and `RapidBlink`)
    NoBlink = 25,
    /// Turns off the `Reverse` attribute
    NoReverse = 27,
    /// Turns off the `Hidden` attribute
    NoHidden = 28,
    /// Turns off the `Strikethrough` attribute
    NoStrikethrough = 29,
}

impl Into<style::Attribute> for Attribute {
    fn into(self) -> style::Attribute {
        match self {
            Attribute::Reset => style::Attribute::Reset,
            Attribute::Bold => style::Attribute::Bold,
            Attribute::Dim => style::Attribute::Dim,
            Attribute::Italic => style::Attribute::Italic,
            Attribute::Underlined => style::Attribute::Underlined,
            Attribute::SlowBlink => style::Attribute::SlowBlink,
            Attribute::RapidBlink => style::Attribute::RapidBlink,
            Attribute::Reverse => style::Attribute::Reverse,
            Attribute::Hidden => style::Attribute::Hidden,
            Attribute::Strikethrough => style::Attribute::CrossedOut,
            Attribute::NoBold => style::Attribute::NoBold,
            Attribute::NormalIntensity => style::Attribute::NormalIntensity,
            Attribute::NoItalic => style::Attribute::NoItalic,
            Attribute::NoUnderline => style::Attribute::NoUnderline,
            Attribute::NoBlink => style::Attribute::NoBlink,
            Attribute::NoReverse => style::Attribute::NoReverse,
            Attribute::NoHidden => style::Attribute::NoHidden,
            Attribute::NoStrikethrough => style::Attribute::NotCrossedOut,
        }
    }
}

impl From<style::Attribute> for Attribute {
    fn from(value: style::Attribute) -> Self {
        match value {
            style::Attribute::Reset => Self::Reset,
            style::Attribute::Bold => Self::Bold,
            style::Attribute::Dim => Self::Dim,
            style::Attribute::Italic => Self::Italic,
            style::Attribute::Underlined => Self::Underlined,
            style::Attribute::SlowBlink => Self::SlowBlink,
            style::Attribute::RapidBlink => Self::RapidBlink,
            style::Attribute::Reverse => Self::Reverse,
            style::Attribute::Hidden => Self::Hidden,
            style::Attribute::CrossedOut => Self::Strikethrough,
            style::Attribute::NoBold => Self::NoBold,
            style::Attribute::NormalIntensity => Self::NormalIntensity,
            style::Attribute::NoItalic => Self::NoItalic,
            style::Attribute::NoUnderline => Self::NoUnderline,
            style::Attribute::NoBlink => Self::NoBlink,
            style::Attribute::NoReverse => Self::Reverse,
            style::Attribute::NoHidden => Self::NoHidden,
            _ => Self::Reset,
        }
    }
}

impl Attribute {
    /// Get the byte of the [Attribute] in the [Attributes] bitset.
    #[inline]
    pub const fn bytes(self) -> u32 {
        1 << ((self as u32) + 1)
    }

    /// Return an iterator of all the possible values of [Attribute].
    pub fn iterator() -> impl Iterator<Item = Attribute> {
        [
            Self::Reset,
            Self::Bold,
            Self::Dim,
            Self::Italic,
            Self::Underlined,
            Self::SlowBlink,
            Self::RapidBlink,
            Self::Reverse,
            Self::Hidden,
            Self::Strikethrough,
            Self::NoBold,
            Self::NormalIntensity,
            Self::NoItalic,
            Self::NoUnderline,
            Self::NoBlink,
            Self::NoReverse,
            Self::NoHidden,
            Self::NoStrikethrough,
        ]
        .iter()
        .copied()
    }
}

/// A list of active attributes
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Attributes(u32);

impl GetAttributes<Attribute> for Attributes {
    fn get_attributes(self) -> Vec<Attribute> {
        Attribute::iterator()
            .into_iter()
            .filter(|a| self.has(*a))
            .collect()
    }
}

// Like in our crate, the u32 underlying attributes in crossterm is private. For this reason, we
// only implement Into for now.
impl From<style::Attributes> for Attributes {
    fn from(attrs: style::Attributes) -> Self {
        let ct_attrs = attrs.get_attributes();

        let mut ret = Self(0);
        ct_attrs
            .iter()
            .map(|a| a.to_owned().into())
            .for_each(|a| ret.set(a));

        ret
    }
}

impl Into<style::Attributes> for Attributes {
    fn into(self) -> style::Attributes {
        let attrs = self.get_attributes();
        let mut ret = style::Attributes::none();
        attrs
            .iter()
            .map(|a| a.to_owned().into())
            .for_each(|a| ret.set(a));

        ret
    }
}

impl From<Attribute> for Attributes {
    fn from(attribute: Attribute) -> Self {
        Self(attribute.bytes())
    }
}

impl From<&[Attribute]> for Attributes {
    fn from(arr: &[Attribute]) -> Self {
        let mut attributes = Attributes::default();
        for &attr in arr {
            attributes.set(attr);
        }
        attributes
    }
}

impl Attributes {
    /// Return empty [Attributes]
    #[inline]
    pub const fn none() -> Self {
        Self(0)
    }

    /// Builder method to add an [Attribute]
    #[inline]
    pub const fn with(self, attribute: Attribute) -> Self {
        Self(self.0 | attribute.bytes())
    }

    /// Builder method to remove an [Attribute]
    #[inline]
    pub const fn without(self, attribute: Attribute) -> Self {
        Self(self.0 & !attribute.bytes())
    }

    /// Set an [Attribute]
    #[inline]
    pub fn set(&mut self, attribute: Attribute) {
        self.0 |= attribute.bytes();
    }

    /// Unset an [Attribute]
    #[inline]
    pub fn unset(&mut self, attribute: Attribute) {
        self.0 &= !attribute.bytes();
    }

    /// Toggle an [Attribute]
    #[inline]
    pub fn toggle(&mut self, attribute: Attribute) {
        self.0 ^= attribute.bytes();
    }

    /// Check if the [Attributes] contain an [Attribute]
    #[inline]
    pub fn has(&self, attribute: Attribute) -> bool {
        self.0 & attribute.bytes() != 0
    }

    /// Extend [Attributes] with another [Attributes]
    #[inline]
    pub fn extend(&mut self, attributes: Attributes) {
        self.0 |= attributes.0
    }

    /// Check if the [Attributes] are empty (no contained [Attribute])
    #[inline]
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Get the intersection of two sets of attributes
    #[inline]
    pub fn intersection(self, other: Self) -> Self {
        // Get the u32 representation of other
        let other: u32 = other.into();
        // Take the bitwise AND and convert it into Self
        (self.0 & other).into()
    }

    /// Patch these attributes with another set of attributes
    #[must_use = "returns a new value"]
    pub fn patch(self, other: Attributes) -> Self {
        let mut attributes = self;
        attributes.extend(other);
        attributes
    }
}

impl From<u32> for Attributes {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl Into<u32> for Attributes {
    fn into(self) -> u32 {
        self.0
    }
}

impl std::ops::Sub for Attributes {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let intersection: u32 = self.intersection(rhs).into();

        (self.0 & !intersection).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::style::Attributes;

    #[test]
    fn intersection() {
        let first: Attributes = 0b00000000000000000000000010110001.into();
        let second: Attributes = 0b00000000000000000000000010010001.into();

        assert_eq!(
            first.intersection(second).0,
            0b00000000000000000000000010010001
        )
    }
}
