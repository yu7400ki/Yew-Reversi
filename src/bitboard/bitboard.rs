use crate::bitboard::types::{Coordinate, Direction, Stone, Turn};

#[derive(PartialEq, Clone, Copy)]

pub struct Bitboard {
    pub black_board: u64,
    pub white_board: u64,
    pub legal_board: u64,
    pub turn: Turn,
    pub pass: bool,
    pub end: bool,
}

impl Bitboard {
    pub fn new() -> Bitboard {
        Bitboard {
            black_board: 34628173824,
            white_board: 68853694464,
            legal_board: 17729692631040,
            turn: Turn::Black,
            pass: false,
            end: false,
        }
    }

    pub fn move_stone(&self, coordinate: Coordinate) -> Result<Self, &str> {
        let mut new_board = self.clone();
        let coordinate = coordinate;

        if !new_board.is_legal(Some(coordinate)) {
            return Ok(new_board);
        }

        new_board.pass = false;
        new_board.flip(&coordinate);
        new_board.change_turn();

        Ok(new_board)
    }

    pub fn bitboard_to_vec(&self) -> Vec<Stone> {
        let mut vec = Vec::new();
        let mut coordinate = Coordinate::from_position(0);

        for _ in 0..64 {
            vec.push(if self.is_black(&coordinate) {
                Stone::Black
            } else if self.is_white(&coordinate) {
                Stone::White
            } else if self.is_legal(Some(coordinate)) {
                let enum_flip = self.enumerate_flip(&coordinate);
                let cnt = enum_flip.count_ones();
                Stone::Legal(cnt)
            } else {
                Stone::Empty
            });
            coordinate = coordinate.next();
        }

        vec
    }

    pub fn count_black(&self) -> u32 {
        self.black_board.count_ones()
    }

    pub fn count_white(&self) -> u32 {
        self.white_board.count_ones()
    }

    fn is_legal(&self, coordinate: Option<Coordinate>) -> bool {
        match coordinate {
            Some(coordinate) => self.legal_board & coordinate.to_bit() != 0,
            None => self.legal_board != 0,
        }
    }

    fn is_black(&self, coordinate: &Coordinate) -> bool {
        let bit = coordinate.to_bit();
        self.black_board & bit == bit
    }

    fn is_white(&self, coordinate: &Coordinate) -> bool {
        let bit = coordinate.to_bit();
        self.white_board & bit == bit
    }

    fn set_legal_board(&mut self) {
        self.legal_board =
            Bitboard::make_legal_board(&self.black_board, &self.white_board, &self.turn);
    }

    fn change_turn(&mut self) {
        match self.turn {
            Turn::Black => self.turn = Turn::White,
            Turn::White => self.turn = Turn::Black,
        }

        self.set_legal_board();

        if self.is_legal(None) {
            return;
        }

        if self.pass {
            self.pass = false;
            self.end = true;
        } else {
            self.pass = true;
            self.change_turn();
        }
    }

    fn enumerate_flip(&self, coordinate: &Coordinate) -> u64 {
        let (own, opponent) = match self.turn {
            Turn::Black => (self.black_board, self.white_board),
            Turn::White => (self.white_board, self.black_board),
        };

        let _lookup = |direction: Direction| -> u64 {
            let result = Bitboard::lookup(&coordinate.to_bit(), &opponent, &direction);
            let shift = direction.to_shift();
            match own & shift(&result) {
                0 => 0,
                _ => result,
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

    fn flip(&mut self, coordinate: &Coordinate) {
        let flip = self.enumerate_flip(&coordinate);
        let bit = coordinate.to_bit();
        match self.turn {
            Turn::Black => {
                self.black_board |= bit | flip;
                self.white_board ^= flip;
            }
            Turn::White => {
                self.white_board |= bit | flip;
                self.black_board ^= flip;
            }
        }
    }

    fn make_legal_board(black: &u64, white: &u64, turn: &Turn) -> u64 {
        let blank = !(black | white);
        let (own, opponent) = match turn {
            Turn::Black => (black, white),
            Turn::White => (white, black),
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
        result
    }

    fn lookup(own: &u64, opponent: &u64, direction: &Direction) -> u64 {
        let shift = direction.to_shift();
        let mask = opponent & direction.to_mask();
        let mut result = mask & shift(&own);
        result |= mask & shift(&result);
        result |= mask & shift(&result);
        result |= mask & shift(&result);
        result |= mask & shift(&result);
        result |= mask & shift(&result);
        result
    }
}
