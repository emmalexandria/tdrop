use crossterm::event::{Event, KeyCode, KeyEvent};
use tdrop::{
    style::Color,
    terminal::Terminal,
    theme::Theme,
    widgets::confirmation::{Confirmation, ConfirmationState},
};

fn main() {
    let mut terminal = Terminal::new(std::io::stdout()).term_width();
    let theme = Theme::empty().info(Color::Blue).success(Color::Green);
    let width = terminal.width;

    let confirmation = Confirmation::new("Do you wish to have a good time?").theme(theme);
    let confirmation_state = ConfirmationState {
        selected: false,
        done: false,
    };

    terminal.render_loop(&confirmation, &width, confirmation_state, |mut state| {
        match crossterm::event::read().unwrap() {
            Event::Key(k) => match k.code {
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
                _ => {}
            },
            _ => {}
        }

        state
    });
}
