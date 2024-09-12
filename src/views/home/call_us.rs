use yew::prelude::*;

#[function_component(CallUs)]
pub fn call_us() -> Html {
    html! {
        <div class="relative w-full h-screen px-5 flex flex-col justify-center gap-10  xl:gap-0 xl:justify-center xl:flex-row-reverse xl:px-40">
            // Container 1
            <div class="flex justify-center items-center">
                <img src="static/img/happy-construction-worker-removed-bg.png" alt="Happy Construction Worker" class="transform scale-125 -translate-x-[60px] md:scale-100 xl:scale-125" />
            </div>
            // Container 2
            <div class="flex flex-col justify-center items-center xl:items-start xl:gap-4">
                <h1 class="text-4xl xl:text-6xl font-bold text-center">{"Zadzwoń Teraz"}</h1>
                <p class="text-center text-xl xl:text-start xl:text-3xl mt-4">{"Zadzwoń do nas, aby usunąć swoje drzwi lub dokonać naprawy!"}</p>

                <div class="flex items-center mt-4 gap-4">
                    // SVG for phone icon
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-10">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 6.75c0 8.284 6.716 15 15 15h2.25a2.25 2.25 0 0 0 2.25-2.25v-1.372c0-.516-.351-.966-.852-1.091l-4.423-1.106c-.44-.11-.902.055-1.173.417l-.97 1.293c-.282.376-.769.542-1.21.38a12.035 12.035 0 0 1-7.143-7.143c-.162-.441.004-.928.38-1.21l1.293-.97c.363-.271.527-.734.417-1.173L6.963 3.102a1.125 1.125 0 0 0-1.091-.852H4.5A2.25 2.25 0 0 0 2.25 4.5v2.25Z" />
                    </svg>

                    // Clickable phone number
                    <a href="tel:+48123456789" class="text-blue-500 hover:text-blue-700 text-xl xl:text-3xl font-semibold">
                        {"+48 123 456 789"}
                    </a>
                </div>
            </div>
        </div>
    }
}
