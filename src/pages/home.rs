use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="relative flex flex-col w-full h-screen">
            // Video Background
            <video autoplay={true} loop={true} muted={true} class="absolute top-0 left-0 w-full h-full object-cover z-[10]">
                <source src="/static/video/header_construction_door.mp4" type="video/mp4" />
                { "Your browser does not support the video tag." }
            </video>

                // Navigation Bar (Example)
                <nav class="navbar bg-base-100 w-full py-4 bg-transparent">
                    <div class="navbar-start pl-8">
                        <div class="flex-1">
                            <a class="btn btn-ghost text-xl">{"DOMEX"}</a>
                        </div>
                    </div>
                    <ul class="navbar-end gap-3 pr-8">
                        <li><a href="/" class="text-white text-xl font-bold">{"Home"}</a></li>
                        <li><a href="/products" class="text-white text-xl font-bold">{"Products"}</a></li>
                        <li><a href="/about-us" class="text-white text-xl font-bold">{"About Us"}</a></li>
                        <li><a href="/contact" class="text-white text-xl font-bold">{"Contact"}</a></li>
                    </ul>
                </nav>

                // Text in the middle of the video
                <div class="text-center">
                    <h1 class="text-5xl text-white font-extrabold">{"Welcome to Our Website"}</h1>
                    <p class="text-2xl text-gray-200 mt-4">{"Innovative Solutions for Your Business"}</p>
                </div>
        </div>
    }
}
