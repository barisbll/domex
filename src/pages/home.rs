use yew::prelude::*;

use crate::components::navbar::NavBar;
use crate::views::home::call_us::CallUs;
use crate::views::home::header::Header;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <NavBar />
            <Header />
            <CallUs />
        </div>
    }
}
