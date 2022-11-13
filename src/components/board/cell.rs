use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct CellProps {
    pub pos: i8,
    pub stone: i8,
}

#[function_component(Cell)]
pub fn cell(props: &CellProps) -> Html {
    html! {
        <div class={format!("cell {}", props.pos)}>
            {props.stone}
        </div>
    }
}
