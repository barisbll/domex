use yew::prelude::*;

use crate::components::{footer::Footer, navbar::NavBar};

#[function_component(AboutUs)]
pub fn about_us() -> Html {
    html! {
        <div class="min-h-screen bg-base-300">
            <NavBar />

            <header class="text-center py-4 text-white">
                <h1 class="text-3xl font-bold text-base-content">{"Kim Jesteśmy?"}</h1>
            </header>

            <div class="container mx-auto px-4 py-8">
                // Responsive layout for image and text
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-8 items-center">
                    // Image section
                    <div class="w-full">
                        <img
                            src="static/img/door-installment.jpg"
                            alt="About us image"
                            class="rounded-lg shadow-lg w-full h-auto object-cover"
                        />
                    </div>

                    // Text section
                    <div class="text-base-content text-lg space-y-4">
                        <p>
                            { "Domex to polska firma z wieloletnim doświadczeniem w sprzedaży i montażu drzwi. Od samego początku naszym celem było dostarczanie najwyższej jakości produktów, które nie tylko spełniają, ale i przewyższają oczekiwania naszych klientów. Dbamy o to, aby nasze drzwi były zarówno estetyczne, jak i funkcjonalne, bo wierzymy, że każdy dom zasługuje na solidne i piękne wejście." }
                        </p>
                        <p>
                            { "Naszą siłą są ludzie – zespół doświadczonych specjalistów, którzy z pasją i zaangażowaniem podchodzą do każdego projektu. Bez względu na to, czy potrzebujesz klasycznych, nowoczesnych, czy też bardziej niestandardowych rozwiązań, zawsze znajdziesz u nas coś dla siebie. Oferujemy profesjonalne doradztwo, dzięki czemu łatwiej podejmiesz decyzję o wyborze idealnych drzwi." }
                        </p>
                        <p>
                            { "W Domex wierzymy, że drzwi to nie tylko element wyposażenia domu – to także symbol bezpieczeństwa, komfortu i stylu. Dlatego dokładamy wszelkich starań, aby nasza oferta odpowiadała na potrzeby zarówno indywidualnych klientów, jak i firm. Zaufaj nam, a stworzymy dla Ciebie przestrzeń, w której poczujesz się jak w domu." }
                        </p>
                    </div>
                </div>
            </div>

            <Footer />
        </div>
    }
}
