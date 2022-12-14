#[derive(PartialEq, Clone, Copy)]
pub enum Turn {
    Black,
    White,
}

impl Turn {
    pub fn opposite(&self) -> Turn {
        match self {
            Turn::Black => Turn::White,
            Turn::White => Turn::Black,
        }
    }
}
