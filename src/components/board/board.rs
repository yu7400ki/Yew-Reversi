use yew::{function_component, html, Callback, Html, Properties, UseStateHandle};

use crate::bitboard::bitboard::Bitboard;
use crate::bitboard::types::Stone;
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
        Callback::from(move |pos: i8| {
            board.set(bitboard.move_stone(pos));
        })
    };

    let generate_cell = |pos: i8, stone: Stone| {
        let on_move_stone = on_move_stone.clone();
        html! {
            <Cell pos={pos} stone={stone} on_move_stone={on_move_stone}/>
        }
    };

    html! {
        <div id="board">
            {
                stones
                .iter()
                .enumerate()
                .map(|item| {
                        let pos = item.0 as i8;
                        let stone = *item.1;
                        generate_cell(pos, stone)
                    }
                ).collect::<Html>()
            }
        </div>
    }
}
