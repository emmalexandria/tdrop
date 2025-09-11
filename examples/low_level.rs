// This example provides examples for using the more low-level features of tdrop directly, such as
// terminal.

use tdrop::{
    layout::Width,
    style::{Color, Stylize},
    terminal::Terminal,
};

fn main() {
    // Create a new Terminal which writes to stdout with a width equal to the terminal
    let mut terminal = Terminal::new(std::io::stdout()).term_width();
    // Create a new Width starting at index 5 that is 3 characters long
    let width = Width::new(5, 3);

    width.write("Hello", &mut terminal);
}
