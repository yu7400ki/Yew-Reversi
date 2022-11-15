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
