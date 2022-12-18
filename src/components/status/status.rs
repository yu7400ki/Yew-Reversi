use yew::{function_component, html, Html, Properties};

use crate::reversi::{BoardBehavior, Game, Turn};

#[derive(Properties, PartialEq)]
pub struct StatusProps<T>
where
    T: BoardBehavior + Clone + PartialEq + Copy,
{
    pub game: Game<T>,
}

#[function_component(Status)]
pub fn cell<T>(props: &StatusProps<T>) -> Html
where
    T: BoardBehavior + Clone + PartialEq + Copy,
{
    let game = &props.game;

    html! {
        <div id="status">
            <p>{format!("●:{}", game.board.count_disc(Turn::Black))}</p>
            <p>{format!("ターン:{}", if game.turn == Turn::Black {"●"} else {"○"})}</p>
            <p>{format!("○:{}", game.board.count_disc(Turn::White))}</p>
        </div>
    }
}
