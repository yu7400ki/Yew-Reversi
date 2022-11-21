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
            <p>{format!("●:{}", bitboard.count_black())}</p>
            {
                match bitboard.end {
                    true => html! {<p>{format!("勝者:{}",
                    match bitboard.winner {
                        Some(Turn::Black) => "●",
                        Some(Turn::White) => "○",
                        None => "引き分け",
                    })}</p>},
                    false => html! {<p>{format!("手番:{}", if bitboard.turn == Turn::Black {"●"} else {"○"})}</p>},
                }
            }
            <p>{format!("○:{}", bitboard.count_white())}</p>
        </div>
    }
}
