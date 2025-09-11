use tdrop::style::{Attribute, Color, Style, Stylize};

fn main() {
    let style = Style::new().with(Color::Red);

    println!("{}", style.apply("Hello"));

    println!(
        "{} {}",
        "This will be bold".attribute(Attribute::Bold),
        "and this italic".attribute(Attribute::Italic)
    );
}
