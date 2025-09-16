//! ![tdrop wordmark](https://github.com/emmalexandria/tdrop/blob/main/media/wordmark.svg?raw=true)
//!
//! # tdrop
//!
//! `tdrop` is a crate for creating CLI-first output.
//!
//! # Quickstart
//! Add `tdrop` as a dependency:
//! `cargo add tdrop`
//!
//!
//! # Introduction
//!
//! `tdrop` is based on a hybrid of immediate rendering and terminal scrollback. Although it is
//! counter-intuitive for CLIs, it puts the terminal in raw mode for the duration of the runtime.
//!
//! ## Basic Components
//! For basic components such as [Span](component::Span) which implement the
//! [Component](component::Component) trait, the [Terminal](terminal::Terminal) will render
//! the component to its active buffer area and then print the buffer.
//!
//! ## Stateful Components
//! For stateful components such as [Confirmation](component::Confirmation) which implement the
//! [StatefulComponent](component::StatefulComponent) trait, the rendering process operates a bit
//! differently. The [Terminal](terminal::Terminal) will re-render the component at a fixed
//! framerate, calling a user-provided closure to update the state of the component.
//!
//! It's important to note that this closure should only poll for events (non-blocking), as
//! otherwise it interferes with the ability of the terminal to respond to control sequences.
//!
//!
//!
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

use std::io::{self, Stdout};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::{backend::CrosstermBackend, terminal::Terminal};

pub mod backend;
pub mod buffer;
pub mod component;
pub mod layout;
pub mod style;
pub mod terminal;
pub mod theme;

pub type DefaultTerminal = Terminal<CrosstermBackend<Stdout>>;

pub fn run<F, R>(f: F) -> R
where
    F: FnOnce(&mut DefaultTerminal) -> R,
{
    let mut terminal = init();
    let result = f(&mut terminal);
    restore();
    result
}

pub fn init() -> DefaultTerminal {
    try_init().expect("failed to init terminal")
}

pub fn try_init() -> io::Result<DefaultTerminal> {
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(std::io::stdout());
    Terminal::new(backend)
}

pub fn restore() {
    if let Err(err) = try_restore() {
        eprintln!("Failed to restore terminal: {err}");
    }
}

pub fn try_restore() -> io::Result<()> {
    disable_raw_mode()?;
    Ok(())
}
