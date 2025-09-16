use crate::layout::Rect;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Viewport {
    Inline(u16),
    Fixed(Rect),
}
