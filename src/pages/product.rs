use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

#[function_component(Product)]
pub fn product(props: &Props) -> Html {
    html! {
        <h1>{"Product "}{props.id.clone()}</h1>
    }
}
