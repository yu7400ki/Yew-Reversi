use rand::Rng;
use yew::prelude::*;

use crate::bitboard::bitboard::Bitboard;
use crate::components::board::board::Board;
use crate::components::status::status::Status;

mod bitboard;
mod components;

#[function_component(App)]
fn app() -> Html {
    let board = use_state(|| Bitboard::new());
    let bitboard = *board;

    use_effect_with_deps(
        {
            let board = board.clone();
            let bitboard = *board;
            move |_| {
                let rand = rand::thread_rng().gen::<f64>();
                if rand > 0.5 {
                    let cpu = bitboard.search().unwrap();
                    let next_board = bitboard.move_stone(cpu).unwrap();
                    board.set(next_board);
                }
                || ()
            }
        },
        (),
    );

    html! {
        <div id="root">
            <Status bitboard={bitboard} />
            <Board board={board}/>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
