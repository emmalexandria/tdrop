//! Provides types for representing output styles

use std::fmt::Display;

use crate::style::{Style, StyledString};

/// Returns if the terminal background is light. If this cannot be determined, return false.
pub fn is_light() -> bool {
    terminal_light::luma().map_or(false, |luma| luma > 0.6)
}
