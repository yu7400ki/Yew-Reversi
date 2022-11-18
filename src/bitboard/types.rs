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
            Direction::Up => |x| x << 8,
            Direction::Down => |x| x >> 8,
            Direction::Left => |x| x << 1,
            Direction::Right => |x| x >> 1,
            Direction::UpLeft => |x| x << 7,
            Direction::UpRight => |x| x << 9,
            Direction::DownLeft => |x| x >> 9,
            Direction::DownRight => |x| x >> 7,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct Coordinate {
    position: u8,
}

impl Coordinate {
    const MIN: u8 = 0;
    const MAX: u8 = 63;

    fn new(position: u8) -> Coordinate {
        if position < Coordinate::MIN || position > Coordinate::MAX {
            panic!("Invalid position: {}", position);
        }
        Coordinate { position }
    }

    pub fn from_position(position: u8) -> Coordinate {
        Coordinate::new(position)
    }

    pub fn to_position(&self) -> u8 {
        self.position
    }

    pub fn from_bit(bit: u64) -> Coordinate {
        let position = bit.leading_zeros() as u8;
        Coordinate::new(position)
    }

    pub fn to_bit(&self) -> u64 {
        1 << (63 - self.position)
    }

    pub fn next(&self) -> Coordinate {
        let position = self.position + 1;
        if position > Coordinate::MAX {
            Coordinate::new(Coordinate::MIN)
        } else {
            Coordinate::new(position)
        }
    }
}
