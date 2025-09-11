use tdrop::{layout::Width, terminal::Terminal, widgets::Span};

fn main() {
    let mut terminal = Terminal::new(std::io::stdout());

    let span = Span::new("Hello");
    let span_width = Width::new(3, 3);

    terminal.render_widget(span, &span_width);
}
