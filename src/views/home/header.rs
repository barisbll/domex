use yew::prelude::*;

use crate::components::navbar::NavBar;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <div class="relative flex flex-col w-full h-screen">
            <div class="absolute top-0 left-0 w-full h-full z-[0]">
                <video autoplay={true} loop={true} muted={true} class="w-full h-full object-cover">
                    <source src="/static/video/header_construction_door.mp4" type="video/mp4" />
                    { "Twoja przeglądarka nie obsługuje tagu wideo." }
                </video>
            </div>

            <div class="absolute top-0 left-0 w-full h-full bg-neutral opacity-75 z-[5]"></div>
            <div class="relative z-[10] flex flex-col w-full h-full">
                <NavBar />

                <div class="flex flex-col justify-center items-center text-center flex-grow">
                    <h1 class="text-2xl md:text-3xl lg:text-4xl xl:text-5xl text-white font-extrabold">{"Drzwi, które Cię wyróżnią"}</h1>
                    <p class="text-md md:text-lg lg:text-xl xl:text-2xl text-gray-200 mt-4">{"Zainwestuj w styl i bezpieczeństwo z naszymi nowoczesnymi rozwiązaniami dla Twojego domu"}</p>
                </div>
            </div>
        </div>
    }
}
