<img align="center" src="https://github.com/emmalexandria/tdrop/blob/main/media/wordmark.svg?raw=true">

---

<p align="center"><i><b>a drop of tui</b></i></p>

`tdrop` is a Rust crate which provides a wide array of functionality for non-interactive terminal applications. 
This includes a widget rendering system (with support for async widgets e.g. progressbars), flex layouts, and terminal styling.  
It uses [crossterm](https://github.com/crossterm-rs/crossterm) as a backend, ensuring it is fully cross platform.

Although it cannot lay a claim to efficiency, I certaintly believe it is best-in-class in terms of the ease with which
one can achieve beautiful cross-platform terminal output.

If you have used [ratatui](https://github.com/ratatui/ratatui), you'll find the interface delightfully similar,
although there are some key differences.

## Table of Contents
- [Features](#features)
- [Getting Started](#getting-started)
  - [Minimal Example](#minimal-example)
  - [Key Concepts](#key-concepts)
- [Documentation](#documentation)

## Features
- Terminal abstraction
- Text styling
- Theming system for e.g. warning, success, and error messages
- Layouts including margin and padding
- Border rendering

## Getting Started

For more in-depth instructions, view the [documentation](www.google.com) or the `examples` directory.

To begin, add `tdrop` as a dependency with `cargo add tdrop`. 

### Minimal Example



### Key Concepts
`tdrop` provides a few key things you'll be interacting with. A few of these are lower level than you'll usually be worried about, such as `Terminal`, which is an abstraction layer over terminal output. `Width` represents a given span of terminal cells starting at `x` with a 
set width. 



