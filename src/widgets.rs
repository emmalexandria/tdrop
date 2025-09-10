use crate::{buffer::Buffer, layout::rect::Rect};

pub trait Widget {
    fn render(self, rect: Rect) -> Buffer;
}

pub trait StatefulWidget: Send + Sync {
    type State;

    fn render(self, rect: Rect, state: Self::State) -> Buffer;
}
