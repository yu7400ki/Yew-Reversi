use crate::bitboard::types::{Coordinate, Direction, Stone, Turn};

const fn pre_compute() -> [i16; 2048] {
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
    const WEIGHTS: [i16; 2048] = pre_compute();

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

    pub fn move_stone(&self, coordinate: &Coordinate) -> Self {
        let mut new_board = self.clone();

        if !new_board.is_legal(Some(coordinate)) {
            return new_board;
        }

        new_board.pass = false;
        new_board.flip(&coordinate);
        new_board.change_turn();

        new_board
    }

    pub fn bitboard_to_vec(&self) -> Vec<Stone> {
        let mut vec = Vec::new();
        let mut coordinate = Coordinate::from_position(0);

        for _ in 0..64 {
            vec.push(self.get_stone(&coordinate));
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

    pub fn evaluate(&self) -> i16 {
        let black_board = self.black_board;
        let white_board = self.white_board;
        let mask = 0b11111111;
        let mut black_score: i16 = 0;
        let mut white_score: i16 = 0;

        for i in 0..8 {
            let black = (black_board >> (i * 8)) & mask;
            let white = (white_board >> (i * 8)) & mask;
            black_score += Bitboard::WEIGHTS[i as usize * 256 + black as usize];
            white_score += Bitboard::WEIGHTS[i as usize * 256 + white as usize];
        }

        match self.turn {
            Turn::Black => black_score - white_score,
            Turn::White => white_score - black_score,
        }
    }

    pub fn search(&self) -> Option<Coordinate> {
        let mut pos = Coordinate::from_position(0);

        let mut max = i16::MIN;
        let mut res: Option<Coordinate> = None;
        for _ in 0..64 {
            if self.is_legal(Some(&pos)) {
                let score = self.predict(&pos);
                if score >= max {
                    max = score;
                    res = Some(pos);
                }
            }
            pos = pos.next();
        }

        res
    }

    fn predict(&self, pos: &Coordinate) -> i16 {
        let mut new_board = self.clone();

        if !new_board.is_legal(Some(pos)) {
            return i16::MIN;
        }

        new_board.flip(&pos);
        new_board.evaluate()
    }

    fn get_stone(&self, coordinate: &Coordinate) -> Stone {
        if self.is_black(coordinate) {
            Stone::Black
        } else if self.is_white(coordinate) {
            Stone::White
        } else if self.is_legal(Some(coordinate)) {
            let enum_flip = self.enumerate_flip(coordinate);
            let cnt = enum_flip.count_ones();
            Stone::Legal(cnt)
        } else {
            Stone::Empty
        }
    }

    fn is_legal(&self, coordinate: Option<&Coordinate>) -> bool {
        match coordinate {
            Some(coordinate) => match self.get_stone(&coordinate) {
                Stone::Legal(_) => true,
                _ => false,
            },
            None => self.legal_board != 0,
        }
    }

    fn is_black(&self, coordinate: &Coordinate) -> bool {
        let stone = self.get_stone(coordinate);
        stone == Stone::Black
    }

    fn is_white(&self, coordinate: &Coordinate) -> bool {
        let stone = self.get_stone(coordinate);
        stone == Stone::White
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
