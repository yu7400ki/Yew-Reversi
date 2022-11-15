use crate::bitboard::types::{Stone, Turn};

struct Mask;

impl Mask {
    pub const VERTICAL: u64 = 0x00ffffffffffff00;
    pub const HORIZON: u64 = 0x7e7e7e7e7e7e7e7e;
    pub const ALLSIDE: u64 = 0x007e7e7e7e7e7e00;
}

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
    fn to_mask(&self) -> u64 {
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

    fn to_shift(&self) -> fn(&u64) -> u64 {
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

pub struct Bitboard {
    pub black_board: u64,
    pub white_board: u64,
    pub legal_board: u64,
    pub turn: Turn,
}

impl Bitboard {
    pub fn new() -> Bitboard {
        Bitboard {
            black_board: 34628173824,
            white_board: 68853694464,
            legal_board: 17729692631040,
            turn: Turn::Black,
        }
    }

    fn lookup(own: &u64, opponent: &u64, direction: &Direction) -> u64 {
        let mask = direction.to_mask();
        let shift = direction.to_shift();
        let mask = mask & opponent;
        let mut result = mask & shift(&own);
        result |= mask & shift(&result);
        result |= mask & shift(&result);
        result |= mask & shift(&result);
        result |= mask & shift(&result);
        result |= mask & shift(&result);
        result
    }

    fn enumerate_flip(&self, pos: &u64) -> u64 {
        let own = match self.turn {
            Turn::Black => self.black_board,
            Turn::White => self.white_board,
        };
        let opponent = match self.turn {
            Turn::Black => self.white_board,
            Turn::White => self.black_board,
        };

        let _lookup = |direction: Direction| -> u64 {
            let result = Bitboard::lookup(&pos, &opponent, &direction);
            let shift = direction.to_shift();
            if own & shift(&result) != 0 {
                result
            } else {
                0
            }
        };

        let mut result = _lookup(Direction::Up);
        result |= _lookup(Direction::Down);
        result |= _lookup(Direction::Left);
        result |= _lookup(Direction::Right);
        result |= _lookup(Direction::UpLeft);
        result |= _lookup(Direction::UpRight);
        result |= _lookup(Direction::DownLeft);
        result |= _lookup(Direction::DownRight);
        result
    }

    fn flip(&mut self, pos: &u64) {
        let flip = self.enumerate_flip(&pos);
        match self.turn {
            Turn::Black => {
                self.black_board |= pos | flip;
                self.white_board ^= flip;
            }
            Turn::White => {
                self.white_board |= pos | flip;
                self.black_board ^= flip;
            }
        }
    }

    fn make_legal_board(&mut self) {
        let blank = !(self.black_board | self.white_board);
        let own = match self.turn {
            Turn::Black => self.black_board,
            Turn::White => self.white_board,
        };
        let opponent = match self.turn {
            Turn::Black => self.white_board,
            Turn::White => self.black_board,
        };

        let _lookup = |direction: Direction| -> u64 {
            let result = Bitboard::lookup(&own, &opponent, &direction);
            let shift = direction.to_shift();
            blank & shift(&result)
        };

        let mut result = _lookup(Direction::Up);
        result |= _lookup(Direction::Down);
        result |= _lookup(Direction::Left);
        result |= _lookup(Direction::Right);
        result |= _lookup(Direction::UpLeft);
        result |= _lookup(Direction::UpRight);
        result |= _lookup(Direction::DownLeft);
        result |= _lookup(Direction::DownRight);
        self.legal_board = result;
    }

    fn change_turn(&mut self) {
        match self.turn {
            Turn::Black => {
                self.turn = Turn::White;
            }
            Turn::White => {
                self.turn = Turn::Black;
            }
        }
        self.make_legal_board();
    }

    fn is_legal(&self, pos: Option<u64>) -> bool {
        match pos {
            Some(pos) => self.legal_board & pos != 0,
            None => self.legal_board != 0,
        }
    }

    pub fn move_stone(&self, pos: i8) -> Self {
        let mut new_board = self.clone();
        let pos: u64 = 1 << (63 - pos);

        if !new_board.is_legal(Some(pos)) {
            return new_board;
        }

        new_board.flip(&pos);
        new_board.change_turn();

        new_board
    }

    pub fn bitboard_to_vec(&self) -> Vec<Stone> {
        let mut vec = Vec::new();
        let blackboard = self.black_board;
        let whiteboard = self.white_board;
        let mut pos: u64 = 1 << 63;

        for _ in 0..64 {
            vec.push(if blackboard & pos == pos {
                Stone::Black
            } else if whiteboard & pos == pos {
                Stone::White
            } else {
                let enum_flip = self.enumerate_flip(&pos);
                let cnt = enum_flip.count_ones();
                Stone::Empty(cnt)
            });
            pos >>= 1;
        }

        vec
    }

    pub fn count_black(&self) -> u32 {
        self.black_board.count_ones()
    }

    pub fn count_white(&self) -> u32 {
        self.white_board.count_ones()
    }
}
