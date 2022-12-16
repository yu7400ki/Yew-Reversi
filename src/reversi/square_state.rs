#[derive(PartialEq, Clone, Copy)]
pub enum SquareState {
    Black,
    White,
    BlackLegal(u32),
    WhiteLegal(u32),
    Empty,
}
