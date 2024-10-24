use crate::components::footer::Footer;
use crate::components::navbar::NavBar;
use crate::data::list_of_outside_doors::LIST_OF_OUTSIDE_DOORS;
use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*; // Import your AppRoute enum

#[function_component(InsideDoors)]
pub fn inside_doors() -> Html {
    html! {
        <div class="min-h-screen bg-base-300">
            <NavBar />
            <header class="text-center py-4 text-white">
                <h1 class="text-3xl font-bold text-base-content">{"Nasze Produkty"}</h1>
            </header>

            <div class="container mx-auto px-4 py-8">
                <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
                    // Loop through the products and render each product as a tile
                    {for LIST_OF_OUTSIDE_DOORS.iter().map(|product| html! {
                        <div class="card shadow-lg hover:shadow-xl bg-base-100 transition rounded-lg text-center">
                            // Use Link from yew_router for client-side routing
                            <Link<Route> to={Route::InsideDoor { id: product.href.to_string() }} classes="no-underline">
                                <div class="p-4">
                                    <img class="w-full h-48 object-contain mx-auto" src={product.img_src} alt={product.name} />
                                </div>
                                <div class="p-4">
                                    <h2 class="text-lg font-bold text-base-content">{product.name}</h2>
                                </div>
                            </Link<Route>>
                        </div>
                    })}
                </div>
            </div>

            <Footer />
        </div>
    }
}
