use tdrop::{
    layout::Width,
    style::{Attribute, Color, Style, Stylize},
    terminal::Terminal,
    widgets::{Line, Span},
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
    terminal.newline();

    let line = Line::default().style(Style::new().on(Color::Blue)).spans([
        Span::raw("Hello")
            .with(Color::Red)
            .attribute(Attribute::Bold),
        Span::raw(" "),
        Span::raw("world!")
            .with(Color::Green)
            .attribute(Attribute::Underlined),
    ]);

    let centerd_area = Width::new(0, 50);
    let line_center = Line::default()
        .style(Style::default().on(Color::Yellow))
        .spans([Span::raw("Hello")])
        .centered();

    terminal.render_widget(line_center, &centerd_area);
}
