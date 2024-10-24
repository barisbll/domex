use crate::components::{footer::Footer, navbar::NavBar};
use crate::data::list_of_metal_doors::LIST_OF_METAL_DOORS;
use yew::prelude::*;

#[function_component(MetalDoors)]
pub fn metal_doors() -> Html {
    html! {
        <div class="min-h-screen bg-base-300">
            <NavBar />
            <header class="text-center py-4 text-white">
                <h1 class="text-3xl font-bold text-base-content">{"Drzwi Metalowe do Domu"}</h1>
            </header>
            <div class="container mx-auto px-4 py-8">
                <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
                    {for LIST_OF_METAL_DOORS.iter().map(|door| html! {
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
