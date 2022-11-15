#[derive(PartialEq, Clone, Copy)]
pub enum Stone {
    Black,
    White,
    Empty(u32),
}

#[derive(PartialEq, Clone, Copy)]
pub enum Turn {
    Black,
    White,
}
