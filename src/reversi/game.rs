use crate::reversi::{Board, Coordinate, Turn};

#[derive(PartialEq, Clone, Copy)]
pub struct Game<T: Board> {
    pub board: T,
    pub turn: Turn,
    pub pass: bool,
    pub end: bool,
    pub winner: Option<Turn>,
}

impl<T> Game<T>
where
    T: Board + Clone,
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

    pub fn move_disc(&self, coordinate: &Coordinate) -> Game<T> {
        let mut new_game = self.clone();

        if !new_game.board.is_legal(coordinate, &new_game.turn) {
            return new_game;
        }

        new_game.board = new_game.board.move_disc(coordinate, &new_game.turn);
        new_game.pass = false;
        new_game.change_turn();

        new_game
    }

    fn change_turn(&mut self) {
        self.turn = self.turn.opposite();

        if self.board.is_end() {
            self.end = true;
            self.winner = self.board.get_winner();
        } else if !self.board.is_able_to_move(&self.turn) {
            self.pass = true;
            self.turn = self.turn.opposite();
        }
    }
}
