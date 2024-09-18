use yew::prelude::*;

use crate::components::{footer::Footer, navbar::NavBar};

#[function_component(Installations)]
pub fn installations() -> Html {
    let images = (1..=24)
        .map(|i| format!("static/img/realisations/realisation_{}.jpeg", i))
        .collect::<Vec<_>>();

    html! {
        <div class="min-h-screen bg-base-300">
            <NavBar />
            <header class="text-center py-4 text-white">
                <h1 class="text-3xl font-bold text-base-content">{"Nasze Realizacji"}</h1>
            </header>
            <div class="container mx-auto px-4 py-8">
                <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
                    {for images.iter().map(|img_src| html! {
                        <div class="card shadow-lg hover:shadow-xl bg-base-100 transition rounded-lg text-center">
                            <a href={img_src.clone()}>
                                <div class="p-4">
                                    <img class="w-full h-48 object-contain mx-auto" src={img_src.clone()} alt={img_src.clone()} />
                                </div>
                            </a>
                        </div>
                    })}
                </div>
            </div>

            <Footer />
        </div>
    }
}
