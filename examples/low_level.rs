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

    // Write to the width on the given terminal
    width.write("Hello", &mut terminal);
    width.writeln("", &mut terminal);

    let longer_width = Width::new(2, 10);

    // Here, we're going to implement rudimentary wrapping with the return value of writeln. This
    // kind of wrapping is implemented by the builtin TextBox widget
    let text = "This is a longer piece of text";
    let mut remaining = text.len();

    while remaining > 0 {
        let written = longer_width.writeln(
            text[text.len() - remaining..].with(Color::Red),
            &mut terminal,
        );
        remaining -= written as usize;
    }
}
