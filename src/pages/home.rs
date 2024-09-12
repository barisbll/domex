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
            <CallUs />
            <WriteUs />
            <Footer />
        </div>
    }
}
