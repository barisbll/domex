use crate::components::{footer::Footer, navbar::NavBar};
use yew::prelude::*; // Ensure the path is correct to where your list is defined.

#[function_component(RoomDoors)]
pub fn room_doors() -> Html {
    html! {
        <div class="min-h-screen bg-base-300">
            <NavBar />
            <header class="text-center py-4 text-white">
                <h1 class="text-3xl font-bold text-base-content">{"Drzwi Pokojowe"}</h1>
            </header>
            <div class="container mx-auto px-4 py-8">
            </div>
            <Footer />
        </div>
    }
}
