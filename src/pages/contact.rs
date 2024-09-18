use yew::prelude::*;

use crate::{
    components::{footer::Footer, navbar::NavBar},
    views::home::write_us::WriteUs,
};

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <div>
        <NavBar />
        <div>
            <h1></h1>
        </div>
        <WriteUs />
        <Footer />
    </div>
    }
}
