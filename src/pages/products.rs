use yew::prelude::*;

use crate::components::footer::Footer;
use crate::components::navbar::NavBar;

use crate::data::list_of_products::LIST_OF_PRODUCTS;

#[function_component(Products)]
pub fn products() -> Html {
    html! {
        <div class="min-h-screen bg-base-300">
        <NavBar />
            // Navbar component can go here
            <header class="text-center py-4 text-white">
                <h1 class="text-3xl font-bold text-base-content">{"Nasze Produkty"}</h1>
            </header>

            <div class="container mx-auto px-4 py-8">
                <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
                    // Loop through the products and render each product as a tile
                    {for LIST_OF_PRODUCTS.iter().map(|product| html! {
                        <div class="card  shadow-lg hover:shadow-xl bg-base-100 transition rounded-lg text-center">
                            <a href={product.href} target="_blank">
                                <div class="p-4">
                                    <img class="w-full h-48 object-contain mx-auto" src={product.img_src} alt={product.name} />
                                </div>
                                <div class="p-4">
                                    <h2 class="text-lg font-bold text-base-content">{product.name}</h2>
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
