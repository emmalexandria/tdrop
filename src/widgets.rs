use std::fmt::Display;

use crate::{buffer::buffer::Buffer, layout::rect::Rect};

/** [[Widget]] is the  */
pub trait Widget {
    fn render(self, frame: Rect) -> String;
}

/// [[StatefulWidget]] supplies facilities for rendering widgets which use some internal state to
/// display interactivity (e.g. progress)
pub trait StatefulWidget {}
