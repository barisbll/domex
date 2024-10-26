use crate::components::footer::Footer;
use crate::components::navbar::NavBar;
use crate::data::list_of_inside_doors::LIST_OF_INSIDE_DOORS;
use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(InsideDoors)]
pub fn inside_doors() -> Html {
    html! {
        <div class="min-h-screen bg-base-300">
            <NavBar />
            <header class="text-center pt-12 pb-4 text-white">
                <h1 class="text-3xl font-bold text-base-content">{"Drzwi Wewnątrzklatkowe"}</h1>
            </header>

            <div class="container mx-auto px-4 py-8">
                <section class="flex flex-col md:flex-row gap-4 md:gap-12 justify-center md:items-start items-center">
                    <div class="md:w-1/2">
                        <h2 class="text-xl font-bold text-base-content py-2">{"Trwałość i Elegancja"}</h2>
                        <p class="text-sm text-base-content">{"Nasze drzwi wewnątrzklatkowe to idealne połączenie solidności i estetyki. Dzięki zastosowaniu najwyższej jakości materiałów, gwarantujemy drzwi, które będą nie tylko trwałe, ale także dodadzą elegancji każdemu wnętrzu. Ich staranne wykonanie sprawia, że są idealnym wyborem zarówno do domów, jak i przestrzeni biurowych."}</p>
                    </div>
                    <div class="md:w-1/2">
                        <h2 class="text-xl font-bold text-base-content py-2">{"Funkcjonalność i Bezpieczeństwo"}</h2>
                        <p class="text-sm text-base-content">{"Oferowane przez nas drzwi wewnątrzklatkowe zapewniają nie tylko estetyczny wygląd, ale również wysoką funkcjonalność i bezpieczeństwo. Dostępne są w różnych wariantach, w tym modele antywłamaniowe oraz dźwiękoszczelne, co czyni je doskonałym wyborem dla osób, które cenią sobie spokój i bezpieczeństwo."}</p>
                    </div>
                </section>
                <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6 py-12">
                    // Loop through the products and render each product as a tile
                    {for LIST_OF_INSIDE_DOORS.iter().map(|product| html! {
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
