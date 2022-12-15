use std::cmp::Ordering;

use crate::reversi::{BoardBehavior, Coordinate, SquareState, Turn};

/* ---------------------------------
    BitBoard
--------------------------------- */

#[derive(PartialEq, Clone, Copy)]
pub struct BitBoard {
    black_board: u64,
    white_board: u64,
    legal_black: u64,
    legal_white: u64,
}

impl BoardBehavior for BitBoard {
    fn new() -> Self {
        BitBoard {
            black_board: 0x0000000810000000,
            white_board: 0x0000001008000000,
            legal_black: 0x0000102004080000,
            legal_white: 0x0000080420100000,
        }
    }

    fn move_disc(&self, coordinate: &Coordinate, turn: &Turn) -> BitBoard {
        self.clone()
    }

    fn is_legal(&self, coordinate: &Coordinate, turn: &Turn) -> bool {
        match turn {
            Turn::Black => self.legal_black & coordinate.to_mask() != 0,
            Turn::White => self.legal_white & coordinate.to_mask() != 0,
        }
    }

    fn is_end(&self) -> bool {
        self.legal_black == 0 && self.legal_white == 0
    }

    fn is_able_to_move(&self, turn: &Turn) -> bool {
        match turn {
            Turn::Black => self.legal_black != 0,
            Turn::White => self.legal_white != 0,
        }
    }

    fn get_winner(&self) -> Option<Turn> {
        let black_count = self.black_board.count_ones();
        let white_count = self.white_board.count_ones();

        match black_count.cmp(&white_count) {
            Ordering::Greater => Some(Turn::Black),
            Ordering::Less => Some(Turn::White),
            Ordering::Equal => None,
        }
    }

    fn to_vec(&self, turn: &Turn) -> Vec<SquareState> {
        (0..64)
            .map(|i| self.get_square_state(&Coordinate::from(i), turn))
            .collect()
    }

    fn evaluate(&self, turn: &Turn) -> i32 {
        let black_count = self.black_board.count_ones() as i32;
        let white_count = self.white_board.count_ones() as i32;

        match turn {
            Turn::Black => black_count - white_count,
            Turn::White => white_count - black_count,
        }
    }

    fn count_black(&self) -> u32 {
        self.black_board.count_ones()
    }

    fn count_white(&self) -> u32 {
        self.white_board.count_ones()
    }
}

impl BitBoard {
    fn get_square_state(&self, coordinate: &Coordinate, turn: &Turn) -> SquareState {
        if self.is_black(coordinate) {
            SquareState::Black
        } else if self.is_white(coordinate) {
            SquareState::White
        } else if self.is_legal(coordinate, turn) {
            SquareState::Legal(0)
        } else {
            SquareState::Empty
        }
    }

    fn is_black(&self, coordinate: &Coordinate) -> bool {
        self.black_board & coordinate.to_mask() != 0
    }

    fn is_white(&self, coordinate: &Coordinate) -> bool {
        self.white_board & coordinate.to_mask() != 0
    }
}

/* ---------------------------------
    Direction
--------------------------------- */

#[derive(PartialEq, Clone, Copy)]
enum Direction {
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
            Direction::UpLeft => Mask::DIAGONAL,
            Direction::UpRight => Mask::DIAGONAL,
            Direction::DownLeft => Mask::DIAGONAL,
            Direction::DownRight => Mask::DIAGONAL,
        }
    }

    pub fn to_shift(&self) -> SHIFTER {
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

/* ---------------------------------
    Mask
--------------------------------- */

struct Mask;

impl Mask {
    pub const VERTICAL: u64 = 0x00ffffffffffff00;
    pub const HORIZON: u64 = 0x7e7e7e7e7e7e7e7e;
    pub const DIAGONAL: u64 = 0x007e7e7e7e7e7e00;
}

/* ---------------------------------
    Shift
--------------------------------- */

type SHIFTER = fn(u64) -> u64;

struct Shift;

impl Shift {
    pub const UP: SHIFTER = |x| x << 8;
    pub const DOWN: SHIFTER = |x| x >> 8;
    pub const LEFT: SHIFTER = |x| x << 1;
    pub const RIGHT: SHIFTER = |x| x >> 1;
    pub const UP_LEFT: SHIFTER = |x| x << 7;
    pub const UP_RIGHT: SHIFTER = |x| x << 9;
    pub const DOWN_LEFT: SHIFTER = |x| x >> 9;
    pub const DOWN_RIGHT: SHIFTER = |x| x >> 7;
}
