use tdrop::backend::Backend;

fn main() {
    let res: std::io::Result<()> = tdrop::run(|term| {
        loop {
            if let Some((ev, exit)) = term.poll_event() {
                if exit {
                    break;
                }
            }
        }
        Ok(())
    });
}
