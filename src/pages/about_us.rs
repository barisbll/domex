use yew::prelude::*;

use crate::components::{footer::Footer, navbar::NavBar};

#[function_component(AboutUs)]
pub fn about_us() -> Html {
    html! {
        <div class="min-h-screen bg-base-300">
            <NavBar />

            <header class="text-center py-4 text-white">
                <h1 class="text-3xl font-bold text-base-content">{"Kto Jesteśmy?"}</h1>
            </header>

            <div class="container mx-auto px-4 py-8">
                // Responsive layout for image and text
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-8 items-center">
                    // Image section
                    <div class="w-full">
                        <img
                            src="https://via.placeholder.com/400"
                            alt="About us image"
                            class="rounded-lg shadow-lg w-full h-auto object-cover"
                        />
                    </div>

                    // Text section
                    <div class="text-base-content text-lg space-y-4">
                        <p>
                            { "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus vehicula purus vel metus placerat, ac feugiat risus dapibus. Integer egestas metus sed odio egestas, et egestas metus blandit. Suspendisse non augue erat." }
                        </p>
                        <p>
                            { "Praesent sollicitudin libero eu magna tristique, at pharetra urna vestibulum. Nullam convallis sem quis justo malesuada, sit amet tempor urna bibendum. Ut at magna nec orci lacinia scelerisque." }
                        </p>
                        <p>
                            { "Curabitur eget quam ac ligula hendrerit posuere id non purus. Sed varius nunc eu felis fermentum, a facilisis metus fermentum. Aliquam erat volutpat." }
                        </p>
                    </div>
                </div>
            </div>

            <Footer />
        </div>
    }
}
