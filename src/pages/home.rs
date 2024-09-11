use yew::prelude::*;

use crate::views::home::header::Header;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <Header />
        </div>
    }
}
