pub mod alignment;
pub mod position;
pub mod rect;
pub mod size;

use crate::{layout::rect::Rect, widgets::Widget};

pub struct Layout<'a> {
    children: Vec<&'a dyn Widget>,
}
