/// [Rect] represents an area within the terminal
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Rect {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}
