#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SquareState {
    Black,
    White,
    Legal(u32),
    Empty,
}
