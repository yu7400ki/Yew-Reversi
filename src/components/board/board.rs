use yew::{function_component, html, Html};

use crate::components::board::cell::Cell;

#[function_component(Board)]
pub fn board() -> Html {
    let stones = vec![0i8; 64];

    html! {
        <div id="board">
            {
                stones
                .iter()
                .enumerate()
                .map(|item| {
                    let pos = item.0 as i8;
                    let stone = *item.1;

                    html! {
                        <Cell pos={pos} stone={stone} />
                    }
                }).collect::<Html>()
            }
        </div>
    }
}
