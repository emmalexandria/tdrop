use crate::{
    buffer::Buffer,
    component::{Component, StatefulComponent},
    layout::{Position, Rect},
};

#[derive(Debug, Hash)]
pub struct Frame<'a> {
    pub(crate) cursor_position: Option<Position>,

    pub(crate) viewport_area: Rect,

    pub(crate) buffer: &'a mut Buffer,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct CompletedFrame<'a> {
    pub buffer: &'a Buffer,
    pub area: Rect,
}

impl Frame<'_> {
    pub const fn area(&self) -> Rect {
        self.viewport_area
    }

    pub fn render_component<C: Component>(&mut self, component: C, area: Rect) {
        component.render(area, self.buffer);
    }

    pub fn render_stateful_component<C>(&mut self, component: C, area: Rect, state: &mut C::State)
    where
        C: StatefulComponent,
    {
        component.render(area, self.buffer, state)
    }
}
