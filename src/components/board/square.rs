use yew::{function_component, html, Callback, Html, MouseEvent, Properties};

use crate::reversi::{Coordinate, SquareState};

#[derive(Properties, PartialEq)]
pub struct SquareProps {
    pub coordinate: Coordinate,
    pub square_state: SquareState,
    pub on_move_disc: Callback<Coordinate>,
}

#[function_component(Square)]
pub fn cell(props: &SquareProps) -> Html {
    let coordinate = props.coordinate;
    let disc = props.square_state;
    let on_move_disc = &props.on_move_disc;

    let onclick = {
        let on_move_disc = on_move_disc.clone();
        let coordinate = coordinate.clone();
        Callback::from(move |_: MouseEvent| {
            on_move_disc.emit(coordinate);
        })
    };

    html! {
        <div class={format!("cell {}", coordinate.to_int())}>
            {
                match disc {
                    SquareState::Black => html! {<p class="stone black">{"●"}</p>},
                    SquareState::White => html! {<p class="stone white">{"●"}</p>},
                    SquareState::Legal(cnt) => html! {<p class="stone legal" {onclick}>{cnt}</p>},
                    SquareState::Empty => html! {<p class="stone empty"></p>},
                }
            }
        </div>
    }
}
