use gloo_console::log;
use gloo_dialogs::alert;
use yew::{function_component, html, Callback, Html, Properties, UseStateHandle};

use crate::components::board::Square;
use crate::reversi::{BoardBehavior, Coordinate, Game, SquareState};

#[derive(Properties, PartialEq)]
pub struct BoardProps<T>
where
    T: BoardBehavior + Clone + PartialEq + Copy,
{
    pub board_state: UseStateHandle<Game<T>>,
}

#[function_component(Board)]
pub fn board<T>(props: &BoardProps<T>) -> Html
where
    T: BoardBehavior + Clone + PartialEq + Copy + 'static,
{
    let board_state = &props.board_state;
    let game = *props.board_state;
    let discs = game.to_vec();

    let on_move_disc = {
        let board_state = board_state.clone();
        Callback::from(move |pos: Coordinate| {
            let mut next_board = game.move_disc(&pos);

            if next_board.pass {
                alert("CPUのターンがパスされます。");
            } else if !next_board.end {
                let cpu = next_board.search().unwrap();
                next_board = next_board.move_disc(&cpu);
                log!("CPU: ", cpu.to_int());
                while next_board.pass {
                    alert("あなたのターンがパスされます。");
                    let cpu = next_board.search().unwrap();
                    next_board = next_board.move_disc(&cpu);
                }
            }

            board_state.set(next_board);
            if next_board.end {
                alert(
                    format!(
                        "黒: {} 白: {}",
                        next_board.board.count_black(),
                        next_board.board.count_white()
                    )
                    .as_str(),
                );
            }
        })
    };

    let generate_cell = |coordinate: Coordinate, square_state: SquareState| {
        let on_move_disc = on_move_disc.clone();
        html! {
            <Square {coordinate} {square_state} {on_move_disc}/>
        }
    };

    html! {
        <div id="board">
            {
                discs
                .iter()
                .enumerate()
                .map(|item| {
                        let coordinate = Coordinate::from(item.0 as u8);
                        let stone = *item.1;
                        generate_cell(coordinate, stone)
                    }
                ).collect::<Html>()
            }
        </div>
    }
}
