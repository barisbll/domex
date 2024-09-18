use yew::prelude::*;

use crate::components::footer::Footer;
use crate::components::navbar::NavBar;

use crate::views::home::call_us::CallUs;
use crate::views::home::header::Header;
use crate::views::home::write_us::WriteUs;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <NavBar />
            <Header />
            <div class="my-20"></div>
            <CallUs />
            <div class="my-16"></div>
            <WriteUs />
            <div class="my-16"></div>
            <Footer />
        </div>
    }
}
