use crossterm::event::{Event, KeyCode, KeyEvent};
use tdrop::{
    backend::{Backend, CrosstermBackend},
    component::confirmation::{Confirmation, ConfirmationState},
    terminal::Terminal,
    DefaultTerminal,
};

fn main() {
    // We use the run function to enter a closure with a mutable reference to the terminal
    // abstraction.
    let res: std::io::Result<()> = tdrop::run(|term| {
        // Here you can do whatever procedural things you like!

        // But if you want to render a stateful widget, enter a loop.
        // Poll events from the terminal to get both any events from the chosen terminal backend
        // and also whether or not an exit signal (CTRL+C) was sent.
        let mut confirmation_state = ConfirmationState { done: false };
        loop {
            let confirmation = Confirmation {};
            term.draw(|frame| {
                frame.render_stateful_component(
                    confirmation,
                    frame.area(),
                    &mut confirmation_state,
                );
            });

            if confirmation_state.done {
                break;
            }
            if let Some((ev, should_exit)) = term.poll_event() {
                if should_exit {
                    exit(term);
                    // This break statement is inserted because `rust-analyzer` doesn't recognise
                    // std::process:exit() as exiting an endless loop
                    break;
                }
                match ev {
                    Event::Key(k) => match k.code {
                        KeyCode::Char('q') => {
                            confirmation_state.done = true;
                            continue;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
        Ok(())
    });
}

fn exit(term: &mut DefaultTerminal) {
    println!("Exiting now!");
    std::process::exit(0)
}
