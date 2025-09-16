use tdrop::{
    backend::{Backend, CrosstermBackend},
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
        // and also whether or not an exit signal was sent.
        loop {
            if let Some((ev, should_exit)) = term.poll_event() {
                if should_exit {
                    exit(term);
                    // This break statement is inserted because `rust-analyzer` doesn't recognise
                    // std::process:exit() as exiting an endless loop
                    break;
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
