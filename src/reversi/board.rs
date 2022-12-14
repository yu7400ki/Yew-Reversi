use crate::reversi::{Coordinate, SquareState, Turn};

pub trait Board {
    fn new() -> Self;
    fn move_disc(&self, coordinate: &Coordinate, turn: &Turn) -> Self;
    fn is_legal(&self, coordinate: &Coordinate, turn: &Turn) -> bool;
    fn is_end(&self) -> bool;
    fn is_able_to_move(&self, turn: &Turn) -> bool;
    fn get_winner(&self) -> Option<Turn>;
    fn to_vec(&self) -> Vec<SquareState>;
}
