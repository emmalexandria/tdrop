use crossterm::style::Stylize;
use tdrop::{layout::rect::Rect, text::line::Line, widgets::Widget, Color, Style};

fn main() {
    let frame = Rect::term().unwrap();

    let mut title = Line::styled("bin_name", Style::new().with(Color::Red));
    title.push("  1.2.5", Style::new().with(Color::Red).italic());

    println!("{}", title.render(frame));
}
