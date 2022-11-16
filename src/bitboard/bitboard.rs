use crate::bitboard::types::{Direction, Stone, Turn};

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

    pub fn move_stone(&self, pos: i8) -> Result<Self, &str> {
        if pos < 0 || pos > 63 {
            return Err("pos is out of range");
        }
        let mut new_board = self.clone();
        let pos: u64 = 1 << (63 - pos);

        if !new_board.is_legal(Some(pos)) {
            return Ok(new_board);
        }

        new_board.pass = false;
        new_board.flip(&pos);
        new_board.change_turn();

        Ok(new_board)
    }

    pub fn bitboard_to_vec(&self) -> Vec<Stone> {
        let mut vec = Vec::new();
        let black_board = self.black_board;
        let white_board = self.white_board;
        let legal_board = self.legal_board;
        let mut pos: u64 = 1 << 63;

        for _ in 0..64 {
            vec.push(if black_board & pos == pos {
                Stone::Black
            } else if white_board & pos == pos {
                Stone::White
            } else if legal_board & pos == pos {
                let enum_flip = self.enumerate_flip(&pos);
                let cnt = enum_flip.count_ones();
                Stone::Legal(cnt)
            } else {
                Stone::Empty
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

    fn enumerate_flip(&self, pos: &u64) -> u64 {
        let (own, opponent) = match self.turn {
            Turn::Black => (self.black_board, self.white_board),
            Turn::White => (self.white_board, self.black_board),
        };

        let _lookup = |direction: Direction| -> u64 {
            let result = Bitboard::lookup(&pos, &opponent, &direction);
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
        let (own, opponent) = match self.turn {
            Turn::Black => (self.black_board, self.white_board),
            Turn::White => (self.white_board, self.black_board),
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
            Turn::Black => self.turn = Turn::White,
            Turn::White => self.turn = Turn::Black,
        }

        self.make_legal_board();

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

    fn is_legal(&self, pos: Option<u64>) -> bool {
        match pos {
            Some(pos) => self.legal_board & pos != 0,
            None => self.legal_board != 0,
        }
    }
}
