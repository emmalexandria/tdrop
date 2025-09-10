use crate::{buffer::Buffer, layout::rect::Rect, Style};

pub trait Widget {
    fn render(self, rect: &Rect) -> Buffer;
}

impl Widget for String {
    fn render(self, rect: &Rect) -> Buffer {
        let mut buffer = Buffer::new(rect);
        buffer.set_string(0, 0, self, Style::default());
        buffer
    }
}

impl<W: Widget> Widget for Option<W> {
    fn render(self, rect: &Rect) -> Buffer {
        if let Some(w) = self {
            return w.render(rect);
        }

        return Buffer::new(rect);
    }
}

pub trait StatefulWidget {
    type State: ?Sized;

    fn render(&mut self, state: &mut Self::State);
}
