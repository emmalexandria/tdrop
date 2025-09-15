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

pub mod backend;
pub mod component;
pub mod layout;
pub mod style;
pub mod terminal;
pub mod theme;
