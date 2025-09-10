use crossterm::style::ContentStyle;

pub trait PatchStyle {
    fn patch<S>(self, other: S) -> Self
    where
        S: Into<Self>,
        Self: Sized;
}

impl PatchStyle for ContentStyle {
    fn patch<S: Into<Self>>(mut self, other: S) -> Self {
        let other: Self = other.into();

        self.foreground_color = other.foreground_color.or(self.foreground_color);
        self.background_color = other.background_color.or(self.background_color);

        self.underline_color = other.underline_color.or(self.underline_color);
        self.attributes.extend(other.attributes);

        self
    }
}
