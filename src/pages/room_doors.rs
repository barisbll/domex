use crate::components::{footer::Footer, navbar::NavBar};
use yew::prelude::*; // Ensure the path is correct to where your list is defined.

#[function_component(RoomDoors)]
pub fn room_doors() -> Html {
    html! {
        <div class="min-h-screen bg-base-300 flex flex-col">
            <NavBar />
            <header class="text-center pt-12 pb-4 text-white">
                <h1 class="text-3xl font-bold text-base-content">{"Drzwi Pokojowe"}</h1>
            </header>
            <div class="container mx-auto px-4 pb-8 pt-4 grow">
            <section class="flex flex-col md:flex-row gap-4 md:gap-12 justify-center md:items-start items-center">
                <div class="md:w-1/2">
                    <h2 class="text-xl font-bold text-base-content py-2">{"Drzwi pokojowe Domex"}</h2>
                    <p class="text-sm text-base-content">{"W Domex oferujemy szeroki wybór drzwi pokojowych, które doskonale komponują się z różnymi stylami aranżacji wnętrz. Niezależnie od tego, czy szukasz nowoczesnych rozwiązań, klasycznej elegancji, czy minimalistycznej prostoty, znajdziesz u nas produkt idealnie dopasowany do swoich potrzeb. Wykorzystujemy materiały najwyższej jakości, co gwarantuje trwałość oraz estetykę."}</p>
                </div>
                <div class="md:w-1/2">
                    <h2 class="text-xl font-bold text-base-content py-2">{"Nasza misja"}</h2>
                    <p class="text-sm text-base-content">{"Misją Domex jest dostarczanie drzwi najwyższej jakości oraz kompleksowa obsługa naszych klientów. Dbamy o to, aby każda decyzja o zakupie drzwi pokojowych była dla Ciebie pewna i komfortowa. Oferujemy także profesjonalny montaż, a nasz zespół fachowców jest gotów sprostać nawet najbardziej wymagającym oczekiwaniom."}</p>
                </div>
            </section>




            <a href="static/pdf/Artus-Katalog-2023-2024-ISOCoated_v2_300-18-08-2023-strona-po-stronie.pdf" target="_blank" rel="noopener noreferrer" class="text-white underline">
                    {"Otwórz katalog (Link)"}
                </a>
            </div>
            <Footer />
        </div>
    }
}
