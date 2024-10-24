use crate::components::{footer::Footer, navbar::NavBar};
use crate::data::list_of_room_doors::LIST_OF_ROOM_DOORS;
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
                <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
                    {for LIST_OF_ROOM_DOORS.iter().map(|door| html! {
                        <div class="card shadow-lg hover:shadow-xl bg-base-100 transition rounded-lg text-center">
                            <a href={door.img_src}>
                                <div class="p-4">
                                    <img class="w-full h-48 object-contain mx-auto" src={door.img_src} alt={door.name} />
                                </div>
                                <h3 class="text-lg font-bold text-base-content p-2">{door.name}</h3>
                            </a>
                        </div>
                    })}
                </div>
            </div>
            <Footer />
        </div>
    }
}
