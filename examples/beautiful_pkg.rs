use std::{thread, time::Duration};

use tdrop::{
    style::{Attribute, Color, Stylize},
    terminal::Terminal,
    theme::Theme,
    widgets::{Line, Span},
};

/// This example implements a visually attractive, width aware package manager simulation.

fn main() {
    let mut terminal = Terminal::new(std::io::stdout()).width(80);
    let width = terminal.width.clone();

    let theme = Theme::empty().primary(Color::Green);

    let header_line = Line::default()
        .style(
            theme
                .primary
                .as_fg()
                .attribute(Attribute::Bold)
                .attribute(Attribute::Underlined),
        )
        .spans([
            Span::raw("pkgdownload "),
            Span::raw("v1.0.0").attribute(Attribute::NormalIntensity),
        ]);

    terminal.render_widget(header_line, &width);
    terminal.newline();
    thread::sleep(Duration::from_secs_f64(5.0));

    terminal.println("Hello");
}
