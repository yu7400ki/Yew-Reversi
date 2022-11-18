use yew::{function_component, html, Callback, MouseEvent, Properties};

use crate::bitboard::types::{Coordinate, Stone};

#[derive(Properties, PartialEq)]
pub struct CellProps {
    pub coordinate: Coordinate,
    pub stone: Stone,
    pub on_move_stone: Callback<Coordinate>,
}

#[function_component(Cell)]
pub fn cell(props: &CellProps) -> Html {
    let coordinate = &props.coordinate;
    let stone = &props.stone;
    let on_move_stone = &props.on_move_stone;

    let onclick = {
        let on_move_stone = on_move_stone.clone();
        let coordinate = coordinate.clone();
        Callback::from(move |_: MouseEvent| {
            on_move_stone.emit(coordinate);
        })
    };

    html! {
        <div class={format!("cell {}", coordinate.to_position())}>
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
