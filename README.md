<img align="center" src="https://github.com/emmalexandria/tdrop/blob/main/assets/wordmark.svg?raw=true">

<p align="center"><i><b>a drop of tui</b></i></p>

---

`tdrop` is a Rust crate for CLI-first high-level terminal output. It provides flexible ways to create high-quality CLI output, ranging from package installers to `--help` printers. 


## Table of Contents
- [Features](#features)
- [Getting Started](#getting-started)
  - [Minimal Example](#minimal-example)
  - [Key Concepts](#key-concepts)
- [Motivation](#motivation)
- [Documentation](#documentation)
- [Child Crates](#child-crates)

## Features
- Terminal abstraction
- Text styling
- Theming system for e.g. warning, success, and error messages
- Layouts including margin and padding
- Border rendering
- Beautiful set of out-of-the-box widgets.

## Getting Started

For more in-depth instructions, view the [documentation](www.google.com) or the `examples` directory.

To begin, add `tdrop` as a dependency with `cargo add tdrop`. 

### Minimal Example



### Key Concepts
`tdrop` provides a few key things you'll be interacting with. A few of these are lower level than you'll usually be worried about, such as `Terminal`, which is an abstraction layer over terminal output. `Width` represents a given span of terminal cells starting at `x` with a 
set width. 

## Motivation
Fundamentally `tdrop` was built due to frustration with Ratatui's inline rendering mode. To be clear, I don't mean to badmouth Ratatui. This project is essentially a fork of it. However, Ratatui makes the assumption that you *always* want a TUI window. `tdrop` allows you to create TUI-like windows at will, render something in them like a progressbar, and then go back to procedural output like a standard CLI.

Like many TUI frameworks, it provides a unified way to render "components" (equivalent to Ratatui widgets), but also allows for 
procedural output and easier once-off rendering of components if all you need is to output a wrapping text-box.

`tdrop` is likely overkill for the vast majority of CLI applications. If all you need is a progress-bar, you're much better off using a crate specifically for that. However, if you're building a CLI with complex output needs like table rendering, progress-bars, user selection, 
and layouts then I truly believe it is best-in-class.

### Advantages
- Procedural output of components.
- On demand buffers for rendering components with changing state.
- Guarantees that your app's output will work cross-platform.

### Disadvantages
- Terminal is kept in raw-mode for the duration of the CLI's runtime.
- No full-screen TUI functionality and no plans to add it.
- Requires a backend (e.g. Crossterm).
- Only supports Crossterm as a backend (for now).


## Documentation

## Child Crates
Although `tdrop` already includes some useful widgets like progressbars and confirmation dialogues, widgets which are more involved or introduce new dependencies have been moved into child crates. A table of these is provided below:

|**Crate**|**Purpose**|
|---------|-----------|
|tdrop_time|Provides widgets for clocks, timers, stopwatches, etc|
|tdrop_table|Provides widgets for rendering fancy tables| 
|tdrop_md|Provides widgets for rendering Markdown text in the terminal|



