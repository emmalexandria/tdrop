use crossterm::style::Stylize;
use tdrop::{layout::rect::Rect, text::span::Span, widgets::Widget, Style};

fn main() {
    let span = Span::styled("Hello", Style::new().red());
    let frame = Rect::term().unwrap();

    println!("{}", span.render(frame))
}
