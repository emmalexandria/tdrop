use tdrop::style::{Color, Stylize};

fn main() {
    tdrop::run(|term| {
        term.println("Hello".with(Color::Red));
        term.println("Hello".with(Color::Red));
        term.println("Hello".with(Color::Red));
        term.println("Hello".with(Color::Red));
    })
}
