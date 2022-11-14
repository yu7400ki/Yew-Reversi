use crate::bitboard::types::{Stone, Turn};

struct Mask;

impl Mask {
    pub const VERTICAL: u64 = 0x7e7e7e7e7e7e7e7e;
    pub const HORIZON: u64 = 0x00ffffffffffff00;
    pub const ALLSIDE: u64 = 0x007e7e7e7e7e7e00;
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

    fn lookup(own: &u64, opponent: &u64, mask: &u64, shift: fn(&u64) -> u64) -> u64 {
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

        let _lookup = |mask: &u64, shift: fn(&u64) -> u64| -> u64 {
            let result = Bitboard::lookup(&pos, &opponent, &mask, shift);
            if own & shift(&result) != 0 {
                result
            } else {
                0
            }
        };

        let mut result = _lookup(&Mask::VERTICAL, |x| x << 1);
        result |= _lookup(&Mask::VERTICAL, |x| x >> 1);
        result |= _lookup(&Mask::HORIZON, |x| x << 8);
        result |= _lookup(&Mask::HORIZON, |x| x >> 8);
        result |= _lookup(&Mask::ALLSIDE, |x| x << 7);
        result |= _lookup(&Mask::ALLSIDE, |x| x >> 7);
        result |= _lookup(&Mask::ALLSIDE, |x| x << 9);
        result |= _lookup(&Mask::ALLSIDE, |x| x >> 9);
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

    fn change_turn(&mut self) {
        match self.turn {
            Turn::Black => {
                self.turn = Turn::White;
            }
            Turn::White => {
                self.turn = Turn::Black;
            }
        }
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
        let mut blackboard = self.black_board;
        let mut whiteboard = self.white_board;
        let mask: u64 = 1 << 63;

        for _ in 0..64 {
            vec.push(if blackboard & mask == mask {
                Stone::Black
            } else if whiteboard & mask == mask {
                Stone::White
            } else {
                Stone::Empty
            });
            blackboard <<= 1;
            whiteboard <<= 1;
        }

        vec
    }
}
