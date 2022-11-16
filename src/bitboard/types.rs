#[derive(PartialEq, Clone, Copy)]
pub enum Stone {
    Black,
    White,
    Legal(u32),
    Empty,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Turn {
    Black,
    White,
}

struct Mask;

impl Mask {
    pub const VERTICAL: u64 = 0x00ffffffffffff00;
    pub const HORIZON: u64 = 0x7e7e7e7e7e7e7e7e;
    pub const ALLSIDE: u64 = 0x007e7e7e7e7e7e00;
}

struct Shift;

impl Shift {
    pub const UP: fn(&u64) -> u64 = |x| x << 8;
    pub const DOWN: fn(&u64) -> u64 = |x| x >> 8;
    pub const LEFT: fn(&u64) -> u64 = |x| x << 1;
    pub const RIGHT: fn(&u64) -> u64 = |x| x >> 1;
    pub const UP_LEFT: fn(&u64) -> u64 = |x| x << 7;
    pub const UP_RIGHT: fn(&u64) -> u64 = |x| x << 9;
    pub const DOWN_LEFT: fn(&u64) -> u64 = |x| x >> 9;
    pub const DOWN_RIGHT: fn(&u64) -> u64 = |x| x >> 7;
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    pub fn to_mask(&self) -> u64 {
        match self {
            Direction::Up => Mask::VERTICAL,
            Direction::Down => Mask::VERTICAL,
            Direction::Left => Mask::HORIZON,
            Direction::Right => Mask::HORIZON,
            Direction::UpLeft => Mask::ALLSIDE,
            Direction::UpRight => Mask::ALLSIDE,
            Direction::DownLeft => Mask::ALLSIDE,
            Direction::DownRight => Mask::ALLSIDE,
        }
    }

    pub fn to_shift(&self) -> fn(&u64) -> u64 {
        match self {
            Direction::Up => Shift::UP,
            Direction::Down => Shift::DOWN,
            Direction::Left => Shift::LEFT,
            Direction::Right => Shift::RIGHT,
            Direction::UpLeft => Shift::UP_LEFT,
            Direction::UpRight => Shift::UP_RIGHT,
            Direction::DownLeft => Shift::DOWN_LEFT,
            Direction::DownRight => Shift::DOWN_RIGHT,
        }
    }
}
