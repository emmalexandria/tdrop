//! ![tdrop wordmark](https://github.com/emmalexandria/tdrop/blob/main/media/wordmark.svg?raw=true)
//!
//! `tdrop` provides flexible and ergonomic output for CLIs. It aims to bring the flexibility of
//! TUI frameworks like [ratatui](https://github.com/ratatui/ratatui) to non-interactive contexts
//! while still allowing for some interactivity in the form of prompts, progressbars, etc.
//!
//! Naming conventions will be very famailiar to any who have used [ratatui](https://docs.rs/ratatui/latest/ratatui).
//! `tdrop` can be used for both complex widget rendering and basic output to the terminal, and engagemenet with
//! its [Terminal](terminal::Terminal) abstraction is optional.
//!
//! ```
//! use tdrop::style::{Stylize, Attribute, Color};
//!
//! fn main() {
//!     let text = "hello".with(Color::Red).on(Color::Green).attribute(Attribute::Bold);
//!
//!     println!("{text}") // This will render as bold red on green.
//! }
//! ```
//!
//! This code will be extremely familiar to anyone who has used [crossterm](https://docs.rs/crossterm/latest/crossterm/).
//! This is because I wholeheartedly ripped off their code. Thanks guys <3. In all seriousness,
//! `crossterm` is currently the rendering backend being used. I wanted to have control over the
//! style types for the long term, but as of right now, they are functionally the `crossterm`
//! types.
//!
//!
//!
//! ```
//!
//! ```
//!
//! ## Important Types
//! * [Terminal](terminal::Terminal) - Provides shared abstraction over the terminal for use by
//! [Widgets](widgets::Widget) and application code.
//! * [Width](layout::Width) - Defines an area of the terminal in which things will be rendered.
//! * [Style](style::Style) - A defined output style including foreground, background, and
//! underline [Color](style::Color) alongside [Attributes](style::Attributes)
//!
//! ## Code Examples
//!

use std::io::{Stdout, Write};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::terminal::Terminal;

#[deny(missing_docs)]
pub mod layout;
#[deny(missing_docs)]
pub mod style;
#[deny(missing_docs)]
pub mod terminal;
#[deny(missing_docs)]
pub mod theme;
#[deny(missing_docs)]
pub mod widgets;

pub fn run<F, R>(f: F) -> R
where
    F: FnOnce(&mut Terminal<Stdout>) -> R,
{
    let mut terminal = init();
    let result = f(&mut terminal);
    restore();
    result
}

pub fn init() -> Terminal<Stdout> {
    try_init().expect("failed to initialise terminal")
}

pub fn try_init() -> std::io::Result<Terminal<Stdout>> {
    enable_raw_mode()?;

    Ok(Terminal::new(std::io::stdout()))
}

pub fn restore() {
    disable_raw_mode();
}
