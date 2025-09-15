//! Provides types for representing output styles
//!
//! This module was designed around simplicity, and as such has limited capability to support
//! arbitrary theming needs. Larger applications with complex theming needs should supplant this
//! module with their own implementation.
//!
//! The primary part of this module is [Theme], which is composed of
//! [ThemeStyles](styles::ThemeStyles) and a collection of [Template](templates::Template).     

mod theme;

pub use theme::Theme;

/// Returns if the terminal background is light. If this cannot be determined, return false.
pub fn is_light() -> bool {
    terminal_light::luma().map_or(false, |luma| luma > 0.6)
}
