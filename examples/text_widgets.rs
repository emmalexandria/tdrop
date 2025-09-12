use tdrop::{
    layout::Width,
    style::{Attribute, Color, Stylize},
    terminal::Terminal,
    widgets::Span,
};

fn main() {
    let mut terminal = Terminal::new(std::io::stdout()).term_width();

    let span: Span = "Hello".with(Color::Red).attribute(Attribute::Bold).into();
    let span_stylize = Span::raw("This will be styled using the stylize trait!")
        .with(Color::Red)
        .on(Color::Magenta);

    let width = Width::new(5, u16::MAX);

    println!("We can render a Span directly with println!");
    println!("{span}");

    println!("Or we can use the terminal and render it to a fixed width");
    terminal.render_widget(span_stylize, &width);
}
