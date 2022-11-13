use crate::bitboard::types::{Stone, Turn};

#[derive(PartialEq, Clone, Copy)]

pub struct Bitboard {
    pub black_board: u64,
    pub white_board: u64,
    pub turn: Turn,
}

impl Bitboard {
    pub fn new() -> Bitboard {
        Bitboard {
            black_board: 34628173824,
            white_board: 68853694464,
            turn: Turn::Black,
        }
    }

    pub fn move_stone(&self, pos: i8) -> Self {
        let mut new_board = self.clone();
        let mask: u64 = 1 << (63 - pos);

        match new_board.turn {
            Turn::Black => {
                new_board.black_board |= mask;
                new_board.turn = Turn::White;
            }
            Turn::White => {
                new_board.white_board |= mask;
                new_board.turn = Turn::Black;
            }
        }

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
