//! Implements the [Color] enum for handling terminal colours.
//!
//! This enum is currently a reimplementation of the `crossterm` enum of the same name,
//! although with some differences in naming

/** ANSI Color */
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    /// Resets the terminal color.
    #[default]
    Reset,
    /// ANSI black. Foreground: 30, background: 40.
    Black,
    /// ANSI red. Foreground: 31, background: 41.
    Red,
    /// ANSI green. Foreground: 32, background: 42.
    Green,
    /// ANSI yellow. Foreground: 33, background: 43.
    Yellow,
    /// ANSI blue. Foreground: 34, background: 44.
    Blue,
    /// ANSI magenta. Foreground: 35, background: 45.
    Magenta,
    /// ANSI cyan. Foreground: 36, background: 46.
    Cyan,
    /// ANSI white. Foreground 37, background 47.
    /// This is not called white becauase white is used for 'light' or 'bright' white
    Gray,
    /// ANSI bright black. Foreground 90, background: 100.
    /// This is usually called bright or light black.
    DarkGray,
    /// ANSI bright red. Foreground: 91, background: 101.
    BrightRed,
    /// ANSI bright green. Foreground 92, background 102.
    BrightGreen,
    /// ANSI bright yellow. Foreground 93, background 103.
    BrightYellow,
    /// ANSI bright blue. Foreground 94. background 104.
    BrightBlue,
    /// ANSI bright magenta. Foreground 95, background 105.
    BrightMagenta,
    /// ANSI bright cyan. Foreground 96, background 106.
    BrightCyan,
    /// ANSI bright white. Foreground 97, background 107.
    White,
    /// An RGB color.
    Rgb { r: u8, g: u8, b: u8 },
    /// An ANSI color. See [256 colors - cheat sheet](https://jonasjacek.github.io/colors/) for more info.
    Indexed(u8),
}

impl From<crossterm::style::Color> for Color {
    fn from(value: crossterm::style::Color) -> Self {
        match value {
            crossterm::style::Color::Reset => Self::Reset,
            crossterm::style::Color::Black => Self::Black,
            crossterm::style::Color::DarkGrey => Self::DarkGray,
            crossterm::style::Color::Red => Self::BrightRed,
            crossterm::style::Color::DarkRed => Self::Red,
            crossterm::style::Color::Green => Self::BrightGreen,
            crossterm::style::Color::DarkGreen => Self::Green,
            crossterm::style::Color::Yellow => Self::BrightYellow,
            crossterm::style::Color::DarkYellow => Self::Yellow,
            crossterm::style::Color::Blue => Self::BrightBlue,
            crossterm::style::Color::DarkBlue => Self::Blue,
            crossterm::style::Color::Magenta => Self::BrightMagenta,
            crossterm::style::Color::DarkMagenta => Self::Magenta,
            crossterm::style::Color::Cyan => Self::BrightCyan,
            crossterm::style::Color::DarkCyan => Self::Cyan,
            crossterm::style::Color::White => Self::White,
            crossterm::style::Color::Grey => Self::Gray,
            crossterm::style::Color::Rgb { r, g, b } => Self::Rgb { r, g, b },
            crossterm::style::Color::AnsiValue(i) => Self::Indexed(i),
        }
    }
}

impl Into<crossterm::style::Color> for Color {
    fn into(self) -> crossterm::style::Color {
        match self {
            Color::Reset => crossterm::style::Color::Reset,
            Color::Black => crossterm::style::Color::Black,
            Color::Red => crossterm::style::Color::DarkRed,
            Color::Green => crossterm::style::Color::DarkGreen,
            Color::Yellow => crossterm::style::Color::DarkYellow,
            Color::Blue => crossterm::style::Color::DarkBlue,
            Color::Magenta => crossterm::style::Color::DarkMagenta,
            Color::Cyan => crossterm::style::Color::DarkCyan,
            Color::Gray => crossterm::style::Color::Grey,
            Color::DarkGray => crossterm::style::Color::DarkGrey,
            Color::BrightRed => crossterm::style::Color::Red,
            Color::BrightGreen => crossterm::style::Color::Green,
            Color::BrightYellow => crossterm::style::Color::Yellow,
            Color::BrightBlue => crossterm::style::Color::Blue,
            Color::BrightMagenta => crossterm::style::Color::Magenta,
            Color::BrightCyan => crossterm::style::Color::Cyan,
            Color::White => crossterm::style::Color::White,
            Color::Rgb { r, g, b } => crossterm::style::Color::Rgb { r, g, b },
            Color::Indexed(i) => crossterm::style::Color::AnsiValue(i),
        }
    }
}
