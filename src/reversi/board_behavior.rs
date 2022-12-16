use crate::reversi::{Coordinate, SquareState, Turn};

pub trait BoardBehavior {
    fn new() -> Self;
    fn is_end(&self) -> bool;
    fn get_winner(&self) -> Option<Turn>;
    fn count_black(&self) -> u32;
    fn count_white(&self) -> u32;
    fn to_vec(&self) -> Vec<SquareState>;
    fn evaluate(&self, turn: &Turn) -> i32;
    fn is_able_to_move(&self, turn: &Turn) -> bool;
    fn is_legal(&self, coordinate: &Coordinate, turn: &Turn) -> bool;
    fn move_disc(&self, coordinate: &Coordinate, turn: &Turn) -> Self;
}
