use std::cmp::Ordering;

use crate::reversi::{BoardBehavior, Coordinate, SquareState, Turn};

/* ---------------------------------
    BitBoard
--------------------------------- */

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct BitBoard {
    board_black: u64,
    board_white: u64,
    legal_black: u64,
    legal_white: u64,
}

impl BoardBehavior for BitBoard {
    fn new() -> Self {
        BitBoard {
            board_black: 0x0000000810000000,
            board_white: 0x0000001008000000,
            legal_black: 0x0000102004080000,
            legal_white: 0x0000080420100000,
        }
    }

    fn move_disc(&self, coordinate: Coordinate, turn: Turn) -> BitBoard {
        let mut new_board = self.clone();

        new_board.flip(coordinate, turn);
        new_board.update_legal();

        new_board
    }

    fn is_legal(&self, coordinate: Coordinate, turn: Turn) -> bool {
        match turn {
            Turn::Black => self.legal_black & coordinate.to_mask() != 0,
            Turn::White => self.legal_white & coordinate.to_mask() != 0,
        }
    }

    fn is_end(&self) -> bool {
        self.legal_black == 0 && self.legal_white == 0
    }

    fn is_able_to_move(&self, turn: Turn) -> bool {
        match turn {
            Turn::Black => self.legal_black != 0,
            Turn::White => self.legal_white != 0,
        }
    }

    fn get_winner(&self) -> Option<Turn> {
        if !self.is_end() {
            return None;
        }

        let black_count = self.board_black.count_ones();
        let white_count = self.board_white.count_ones();

        match black_count.cmp(&white_count) {
            Ordering::Greater => Some(Turn::Black),
            Ordering::Less => Some(Turn::White),
            Ordering::Equal => None,
        }
    }

    fn to_vec(&self, turn: Turn) -> Vec<SquareState> {
        (0..64)
            .map(|i| Coordinate::from(i))
            .map(|c| self.get_square_state(c, turn))
            .collect()
    }

    fn evaluate(&self, turn: Turn) -> i16 {
        let board_black = self.board_black;
        let board_white = self.board_white;
        let mask = 0b11111111;
        let mut black_score: i16 = 0;
        let mut white_score: i16 = 0;

        for i in 0..8 {
            let black = (board_black >> (i * 8)) & mask;
            let white = (board_white >> (i * 8)) & mask;
            black_score += BitBoard::WEIGHTS[i as usize * 256 + black as usize];
            white_score += BitBoard::WEIGHTS[i as usize * 256 + white as usize];
        }

        match turn {
            Turn::Black => black_score - white_score,
            Turn::White => white_score - black_score,
        }
    }

    fn count_disc(&self, turn: Turn) -> u32 {
        match turn {
            Turn::Black => self.board_black.count_ones(),
            Turn::White => self.board_white.count_ones(),
        }
    }
}

impl BitBoard {
    const WEIGHTS: [i16; 2048] = pre_compute_weight();

    fn get_square_state(&self, coordinate: Coordinate, turn: Turn) -> SquareState {
        if self.is_black(coordinate) {
            SquareState::Black
        } else if self.is_white(coordinate) {
            SquareState::White
        } else if self.is_legal(coordinate, turn) {
            let legals = self.enumerate_flip(coordinate, turn).count_ones();
            SquareState::Legal(legals)
        } else {
            SquareState::Empty
        }
    }

    fn is_black(&self, coordinate: Coordinate) -> bool {
        self.board_black & coordinate.to_mask() != 0
    }

    fn is_white(&self, coordinate: Coordinate) -> bool {
        self.board_white & coordinate.to_mask() != 0
    }

    fn enumerate_flip(&self, coordinate: Coordinate, turn: Turn) -> u64 {
        let (own, opponent) = match turn {
            Turn::Black => (self.board_black, self.board_white),
            Turn::White => (self.board_white, self.board_black),
        };

        let lookup = |direction: Direction| -> u64 {
            let shift = direction.to_shift();
            let mask = coordinate.to_mask();
            let result = BitBoard::lookup(mask, opponent, direction);
            match own & shift(result) {
                0 => 0,
                _ => result,
            }
        };

        let mut result = lookup(Direction::Up);
        result |= lookup(Direction::Down);
        result |= lookup(Direction::Left);
        result |= lookup(Direction::Right);
        result |= lookup(Direction::UpLeft);
        result |= lookup(Direction::UpRight);
        result |= lookup(Direction::DownLeft);
        result |= lookup(Direction::DownRight);

        result
    }

    fn flip(&mut self, coordinate: Coordinate, turn: Turn) {
        let flip = self.enumerate_flip(coordinate, turn);
        let moved_disc = coordinate.to_mask();
        match turn {
            Turn::Black => {
                self.board_black |= moved_disc | flip;
                self.board_white ^= flip;
            }
            Turn::White => {
                self.board_white |= moved_disc | flip;
                self.board_black ^= flip;
            }
        }
    }

    fn update_legal(&mut self) {
        let blank = !(self.board_black | self.board_white);

        let generate = |own: u64, opponent: u64| -> u64 {
            let lookup = |direction: Direction| -> u64 {
                let shift = direction.to_shift();
                let result = BitBoard::lookup(own, opponent, direction);
                blank & shift(result)
            };

            let mut result = lookup(Direction::Up);
            result |= lookup(Direction::Down);
            result |= lookup(Direction::Left);
            result |= lookup(Direction::Right);
            result |= lookup(Direction::UpLeft);
            result |= lookup(Direction::UpRight);
            result |= lookup(Direction::DownLeft);
            result |= lookup(Direction::DownRight);

            result
        };

        self.legal_black = generate(self.board_black, self.board_white);
        self.legal_white = generate(self.board_white, self.board_black);
    }

    fn lookup(own: u64, opponent: u64, direction: Direction) -> u64 {
        let shift = direction.to_shift();
        let mask = opponent & direction.to_mask();
        let mut result = mask & shift(own);
        result |= mask & shift(result);
        result |= mask & shift(result);
        result |= mask & shift(result);
        result |= mask & shift(result);
        result |= mask & shift(result);
        result
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

/* ---------------------------------
    PreCompute
--------------------------------- */

const fn pre_compute_weight() -> [i16; 2048] {
    let weights: [i16; 64] = [
        100, -40, 20, 5, 5, 20, -40, 100, -40, -80, -1, -1, -1, -1, -80, -40, 20, -1, 5, 1, 1, 5,
        -1, -20, 5, -1, 1, 0, 0, 1, -1, 5, 5, -1, 1, 0, 0, 1, -1, 5, 20, -1, 5, 1, 1, 5, -1, -20,
        -40, -80, -1, -1, -1, -1, -80, -40, 100, -40, 20, 5, 5, 20, -40, 100,
    ];

    let mut memo = [0i16; 2048];

    let mut y: usize = 0;
    while y < 8 {
        let mut x: usize = 0;
        while x < 256 {
            let mut sum: i16 = 0;
            let mut i: usize = 0;
            while i < 8 {
                let bit = (x >> i) & 1;
                sum += weights[y * 8 + i] * bit as i16;
                i += 1;
            }
            memo[y * 256 + x] = sum;
            x += 1;
        }
        y += 1;
    }

    memo
}
