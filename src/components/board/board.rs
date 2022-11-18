use gloo_dialogs::alert;
use yew::{function_component, html, Callback, Html, Properties, UseStateHandle};

use crate::bitboard::bitboard::Bitboard;
use crate::bitboard::types::{Coordinate, Stone, Turn};
use crate::components::board::cell::Cell;

#[derive(Properties, PartialEq)]
pub struct BoardProps {
    pub board: UseStateHandle<Bitboard>,
}

#[function_component(Board)]
pub fn board(props: &BoardProps) -> Html {
    let board = &props.board;
    let bitboard = *props.board;
    let stones = bitboard.bitboard_to_vec();

    let on_move_stone = {
        let board = board.clone();
        Callback::from(move |coordinate: Coordinate| {
            let next_board = bitboard.move_stone(coordinate).unwrap_or(bitboard.clone());
            if next_board.end {
                alert(
                    format!(
                        "黒: {} 白: {}",
                        next_board.count_black(),
                        next_board.count_white()
                    )
                    .as_str(),
                );
            } else if next_board.pass {
                alert(
                    format!(
                        "{}のターンがパスされました",
                        match next_board.turn {
                            Turn::Black => "白",
                            Turn::White => "黒",
                        }
                    )
                    .as_str(),
                );
            }
            board.set(next_board);
        })
    };

    let generate_cell = |coordinate: Coordinate, stone: Stone| {
        let on_move_stone = on_move_stone.clone();
        html! {
            <Cell {coordinate} {stone} {on_move_stone}/>
        }
    };

    html! {
        <div id="board">
            {
                stones
                .iter()
                .enumerate()
                .map(|item| {
                        let coordinate = Coordinate::from_position(item.0 as u8);
                        let stone = *item.1;
                        generate_cell(coordinate, stone)
                    }
                ).collect::<Html>()
            }
        </div>
    }
}
