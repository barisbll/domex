use crate::components::{footer::Footer, navbar::NavBar};
use yew::prelude::*; // Ensure the path is correct to where your list is defined.

#[function_component(SteelDoors)]
pub fn steel_doors() -> Html {
    html! {
        <div class="min-h-screen bg-base-300 flex flex-col">
            <NavBar />
            <header class="text-center pt-12 pb-4 text-white">
                <h1 class="text-3xl font-bold text-base-content">{"Drzwi Stalowe"}</h1>
            </header>
            <div class="container mx-auto px-4 pb-8 pt-4 grow">
            <section class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                    <h2 class="text-xl font-bold text-base-content py-2 pb-2">{"Drzwi stalowe do Domu"}</h2>
                    <p class="text-sm text-base-content pb-4">{"Nasze drzwi stalowe to doskonałe połączenie funkcjonalności, estetyki i najwyższego poziomu bezpieczeństwa, spełniające rygorystyczne normy branżowe."}</p>
                    <p class="text-sm text-base-content pb-4">{"W Domex stawiamy na trwałość i niezawodność. Nasze drzwi stalowe są wykonane z najwyższej jakości stali, co zapewnia nie tylko elegancki wygląd, ale także długotrwałą wytrzymałość. Oferujemy możliwość personalizacji koloru, rozmiaru oraz dodatkowych opcji, aby drzwi idealnie odpowiadały Twoim potrzebom."}</p>
                </div>
                <div>
                    <p class="text-sm text-base-content">{"Przekonaj się, dlaczego nasze drzwi stalowe zyskały uznanie klientów. Skorzystaj z naszego doświadczenia, profesjonalnej obsługi i szerokiej oferty, aby poprawić funkcjonalność, estetykę i bezpieczeństwo swojego domu. Skontaktuj się z nami już dziś, a nasi specjaliści pomogą Ci wybrać idealne drzwi dopasowane do Twoich oczekiwań."}</p>
                </div>
            </section>

            <section class="flex flex-col md:flex-row gap-8 justify-center  items-center md:items-center md:justify-start pt-8 md:pt-16">
                <div class="md:w-1/2">
                    <img src="static/img/catalogues/stalprodukt-catalogue.png" alt="Drzwi pokojowe katalog" class="rounded-lg shadow-lg" />
                </div>
                <div class="md:w-1/2 flex flex-col justify-center items-center md:items-start gap-2">
                    <h2 class="text-xl md:text-2xl font-bold text-base-content py-2">{"Katalog drzwi firmy Stalprodukt"}</h2>
                        <a href="static/pdf/stalproduckt-catalogue.pdf" target="_blank" rel="noopener noreferrer" class="btn btn-primary">
                        {"Otwórz katalog"}
                        </a>
                </div>
            </section>
            </div>
            <Footer />
        </div>
    }
}
