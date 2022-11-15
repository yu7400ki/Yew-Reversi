use yew::{function_component, html, Callback, MouseEvent, Properties};

use crate::bitboard::types::Stone;

#[derive(Properties, PartialEq)]
pub struct CellProps {
    pub pos: i8,
    pub stone: Stone,
    pub on_move_stone: Callback<i8>,
}

#[function_component(Cell)]
pub fn cell(props: &CellProps) -> Html {
    let pos = &props.pos;
    let stone = &props.stone;
    let on_move_stone = &props.on_move_stone;

    let onclick = {
        let on_move_stone = on_move_stone.clone();
        let pos = pos.clone();
        Callback::from(move |_: MouseEvent| {
            on_move_stone.emit(pos);
        })
    };

    html! {
        <div class={format!("cell {}", pos)}>
            {
                match stone {
                    Stone::Black => html! {<p class="stone black">{"●"}</p>},
                    Stone::White => html! {<p class="stone white">{"●"}</p>},
                    Stone::Legal(cnt) => html! {<p class="stone legal" {onclick}>{cnt}</p>},
                    Stone::Empty => html! {<p class="stone empty"></p>},
                }
            }
        </div>
    }
}
