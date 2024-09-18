use yew::prelude::*;

use crate::{
    components::{footer::Footer, navbar::NavBar},
    views::home::{call_us::CallUs, write_us::WriteUs},
};

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <div>
            <NavBar />
            <CallUs />
            <div class="my-16"></div>
            <WriteUs />
            <div class="my-16"></div>
            <Footer />
        </div>
    }
}
