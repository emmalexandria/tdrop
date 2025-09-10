use crossterm::style::Stylize;
use tpop::{layout::rect::Rect, text::span::Span, widgets::Widget, Style};

fn main() {
    let text = Rect::new(0, 0, 15, 1);

    let span = Span::from("Hello").style(Style::new().red());

    println!("{}", span.render(&text));
}

