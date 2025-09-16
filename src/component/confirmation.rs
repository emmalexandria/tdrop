use crate::{
    component::StatefulComponent,
    style::{Color, Style},
};

pub struct ConfirmationState {
    pub done: bool,
}

pub struct Confirmation {}

impl StatefulComponent for Confirmation {
    type State = ConfirmationState;

    fn render(
        self,
        area: crate::layout::Rect,
        buffer: &mut crate::buffer::Buffer,
        state: &mut Self::State,
    ) {
        if state.done {
            buffer.set_style(area, Style::new().fg(Color::Red));

            buffer[(area.x, area.y)].set_symbol("d");
        } else {
            buffer.set_style(area, Style::new().bg(Color::Green));
            buffer[(area.x, area.y)].set_symbol("r");
        }
    }
}
