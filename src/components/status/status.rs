use yew::{function_component, html, Properties};

use crate::bitboard::bitboard::Bitboard;
use crate::bitboard::types::Turn;

#[derive(Properties, PartialEq)]
pub struct StatusProps {
    pub bitboard: Bitboard,
}

#[function_component(Status)]
pub fn cell(props: &StatusProps) -> Html {
    let bitboard = &props.bitboard;

    html! {
        <div id="status">
            <p>{format!("●: {}", bitboard.count_black())}</p>
            <p>{format!("ターン: {}", if bitboard.turn == Turn::Black {"●"} else {"○"})}</p>
            <p>{format!("○: {}", bitboard.count_white())}</p>
        </div>
    }
}
