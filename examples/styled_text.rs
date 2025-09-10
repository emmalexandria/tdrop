use crossterm::style::Stylize;
use tdrop::{layout::rect::Rect, text::span::Span, widgets::Widget, Style};

fn main() {
    let area = Rect::new(0, 0, 5, None).expands(true);

    let span = Span::styled("Hello", Style::new().red());

    println!("{}", span.render(area));
}
