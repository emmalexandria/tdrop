use tdrop::{
    style::{Color, Stylize},
    terminal::Terminal,
};

fn main() {
    let mut terminal = Terminal::new(std::io::stdout(), None);

    terminal.println("Hello".with(Color::Red));
    terminal.println("Hello".with(Color::Red));

    println!("{}", "HI".with(Color::Green));

    terminal.println("Hi");
}
