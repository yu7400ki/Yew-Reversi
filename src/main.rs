use yew::prelude::*;

use crate::bitboard::bitboard::Bitboard;
use crate::components::board::board::Board;

mod bitboard;
mod components;

#[function_component(App)]
fn app() -> Html {
    let board = use_state(|| Bitboard::new());

    html! {
        <div id="root">
            <Board board={board}/>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
