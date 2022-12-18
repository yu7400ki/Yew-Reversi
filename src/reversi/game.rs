use crate::reversi::{BoardBehavior, Coordinate, Turn};

#[derive(PartialEq, Clone, Copy)]
pub struct Game<T>
where
    T: BoardBehavior + Clone + Copy,
{
    pub board: T,
    pub turn: Turn,
    pub pass: bool,
    pub end: bool,
    pub winner: Option<Turn>,
}

impl<T> Game<T>
where
    T: BoardBehavior + Clone + Copy,
{
    pub fn from(board: T) -> Game<T> {
        Game {
            board,
            turn: Turn::Black,
            pass: false,
            end: false,
            winner: None,
        }
    }

    pub fn move_disc(&self, coordinate: Coordinate) -> Game<T> {
        let mut new_game = self.clone();

        if !new_game.board.is_legal(coordinate, new_game.turn) {
            panic!("Illegal move");
        }

        new_game.board = new_game.board.move_disc(coordinate, new_game.turn);
        new_game.change_turn();

        new_game
    }

    pub fn search(&self) -> Option<Coordinate> {
        (0..64)
            .map(|i| Coordinate::from(i))
            .filter(|&coordinate| self.board.is_legal(coordinate, self.turn))
            .map(|coordinate| (coordinate, self.predict(coordinate)))
            .max_by_key(|(_, score)| *score)
            .map(|(coordinate, _)| coordinate)
    }

    fn predict(&self, coordinate: Coordinate) -> i16 {
        let board = self.board;

        if !board.is_legal(coordinate, self.turn) {
            return i16::MIN;
        }

        let new_board = board.move_disc(coordinate, self.turn);
        new_board.evaluate(self.turn)
    }

    fn change_turn(&mut self) {
        self.turn = self.turn.opposite();
        self.pass = false;

        if self.board.is_end() {
            self.end = true;
            self.winner = self.board.get_winner();
        } else if !self.board.is_able_to_move(self.turn) {
            self.pass = true;
            self.turn = self.turn.opposite();
        }
    }
}
