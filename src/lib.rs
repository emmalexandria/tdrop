//! `tdrop` provides flexible and ergonomic output for CLIs. It aims to bring the flexibility of
//! TUI frameworks like [ratatui](https://github.com/ratatui/ratatui) to non-interactive contexts
//! while still allowing for some interactivity in the form of prompts, progressbars, etc.
mod buffer;
pub mod layout;
/// The text module provides capabilities for rendering styled text, both for
/// external use and use within `tdrop`.
pub mod text;
pub mod widgets;

pub type Color = crossterm::style::Color;
pub type Style = crossterm::style::ContentStyle;
pub type Attribute = crossterm::style::Attribute;
pub type Attributes = crossterm::style::Attributes;
