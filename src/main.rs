use yew::prelude::*;
use components::board::board::Board;

mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div id="root">
            <Board />
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
