use rand::Rng;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::bitboard::bitboard::Bitboard;
use crate::components::board::board::Board;
use crate::components::status::status::Status;
use crate::route::Route;

mod bitboard;
mod components;
mod route;

#[derive(Properties, PartialEq)]
struct HomeProps {
    pub board: UseStateHandle<Bitboard>,
}

#[function_component(Home)]
fn home(props: &HomeProps) -> Html {
    let board = props.board.clone();
    let bitboard = *props.board;

    html! {
        <div id="root">
            <Status bitboard={bitboard} />
            <Board board={board}/>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ResultProps {
    pub board: UseStateHandle<Bitboard>,
}

#[function_component(Result)]
fn result(props: &ResultProps) -> Html {
    let board = props.board.clone();
    let bitboard = *props.board;

    html! {
        <div id="root">
            <Status bitboard={bitboard} />
            <Board board={board}/>
        </div>
    }
}

fn switch(route: &Route, board: UseStateHandle<Bitboard>) -> Html {
    match route {
        Route::Home => html! {<Home board={board} />},
        Route::Result => html! {<Result board={board} />},
        Route::NotFound => html! {<p>{"404"}</p>},
    }
}

#[function_component(App)]
fn app() -> Html {
    let board = use_state(|| Bitboard::new());

    use_effect_with_deps(
        {
            let board = board.clone();
            let bitboard = *board;
            move |_| {
                let rand = rand::thread_rng().gen::<f64>();
                if rand > 0.5 {
                    let cpu = bitboard.search().unwrap();
                    let next_board = bitboard.move_stone(&cpu);
                    board.set(next_board);
                }
                || ()
            }
        },
        (),
    );

    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(move |route: &Route| {
                let board = board.clone();
                switch(route, board)
            })} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
