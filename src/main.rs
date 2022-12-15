use rand::Rng;
use yew::prelude::*;

use crate::components::board::Board;
use crate::components::status::Status;
use crate::reversi::{BitBoard, BoardBehavior, Game};

mod bitboard;
mod components;
mod reversi;

#[function_component(App)]
fn app() -> Html {
    let board_state = use_state(|| Game::from(BitBoard::new()));
    let game = *board_state;

    use_effect_with_deps(
        {
            let board_state = board_state.clone();
            let game = *board_state;
            move |_| {
                let rand = rand::thread_rng().gen::<f64>();
                if rand > 1.0 {
                    let cpu = game.search().unwrap();
                    let next_board = game.move_disc(&cpu);
                    board_state.set(next_board);
                }
                || ()
            }
        },
        (),
    );

    html! {
        <div id="root">
            <Status<BitBoard> {game} />
            <Board<BitBoard> {board_state}/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
