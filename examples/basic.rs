use tdrop::backend::Backend;

fn main() {
    let res: std::io::Result<()> = tdrop::run(|term| term.backend_mut().append_lines(3));
}
