use std::process;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use tdrop::{
    style::Color,
    terminal::Terminal,
    theme::Theme,
    widgets::confirmation::{Confirmation, ConfirmationState},
};

fn main() {
    let mut terminal = Terminal::new(std::io::stdout()).term_width();
    terminal.init();
    let theme = Theme::empty().info(Color::Blue).success(Color::Green);
    let width = terminal.width;

    let confirmation = Confirmation::new("Do you wish to have a good time?").theme(theme);
    let confirmation_state = ConfirmationState {
        selected: false,
        done: false,
    };

    terminal.render_loop(&confirmation, &width, confirmation_state, |mut state| {
        match crossterm::event::read().unwrap() {
            Event::Key(KeyEvent {
                code,
                modifiers,
                kind: _,
                state: _,
            }) => match code {
                KeyCode::Char('y') => {
                    state.selected = true;
                    state.done = true;
                }
                KeyCode::Char('n') => {
                    state.selected = false;
                    state.done = true;
                }
                KeyCode::Left | KeyCode::Right => state.selected = !state.selected,
                KeyCode::Enter => state.done = true,
                KeyCode::Char('c') => {
                    if modifiers.contains(KeyModifiers::CONTROL) {
                        process::exit(0);
                    }
                }
                _ => {}
            },
            _ => {}
        }

        state
    });

    terminal.cleanup();
}
