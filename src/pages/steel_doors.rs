use crate::components::{footer::Footer, navbar::NavBar};
use yew::prelude::*;

#[function_component(SteelDoors)]
pub fn metal_doors() -> Html {
    html! {
        <div class="min-h-screen bg-base-300">
            <NavBar />
            <header class="text-center py-4 text-white">
                <h1 class="text-3xl font-bold text-base-content">{"Drzwi Metalowe do Domu"}</h1>
            </header>
            <div class="container mx-auto px-4 py-8">
            </div>
            <Footer />
        </div>
    }
}
