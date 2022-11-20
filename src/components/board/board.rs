use gloo_console::log;
use gloo_dialogs::alert;
use yew::{function_component, html, Callback, Html, Properties, UseStateHandle};

use crate::bitboard::bitboard::Bitboard;
use crate::bitboard::types::{Coordinate, Stone};
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
        Callback::from(move |pos: Coordinate| {
            let mut next_board = bitboard.move_stone(pos).unwrap_or(bitboard.clone());

            log!("evaluation: ", next_board.evaluate());

            if next_board.pass {
                alert("CPUのターンがパスされます。");
            } else if !next_board.end {
                let cpu = next_board.search().unwrap();
                next_board = next_board.move_stone(cpu).unwrap();
                log!("CPU: ", cpu.to_position());
                while next_board.pass {
                    alert("あなたのターンがパスされます。");
                    let cpu = next_board.search().unwrap();
                    next_board = next_board.move_stone(cpu).unwrap();
                }
            }

            board.set(next_board);
            if next_board.end {
                alert(
                    format!(
                        "黒: {} 白: {}",
                        next_board.count_black(),
                        next_board.count_white()
                    )
                    .as_str(),
                );
            }
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
