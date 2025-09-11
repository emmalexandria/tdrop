//! `tdrop` provides flexible and ergonomic output for CLIs. It aims to bring the flexibility of
//! TUI frameworks like [ratatui](https://github.com/ratatui/ratatui) to non-interactive contexts
//! while still allowing for some interactivity in the form of prompts, progressbars, etc.

//! Implementation of ANSI colours and attributes for text styling

#[deny(missing_docs)]
pub mod layout;
#[deny(missing_docs)]
pub mod style;
#[deny(missing_docs)]
pub mod terminal;
#[deny(missing_docs)]
pub mod widgets;
