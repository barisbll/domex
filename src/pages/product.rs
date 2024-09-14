use yew::prelude::*;

use crate::components::navbar::NavBar;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

#[function_component(Product)]
pub fn product(props: &Props) -> Html {
    html! {
        <div class="min-h-screen bg-base-300">
            <NavBar />
            <h1>{"Product "}{props.id.clone()}</h1>
        </div>
    }
}
