use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="relative flex flex-col w-full h-screen">
            // Video background container
            <div class="absolute top-0 left-0 w-full h-full z-[0]">
                <video autoplay={true} loop={true} muted={true} class="w-full h-full object-cover">
                    <source src="/static/video/header_construction_door.mp4" type="video/mp4" />
                    { "Your browser does not support the video tag." }
                </video>
            </div>

            // Black overlay for darkening effect
            <div class="absolute top-0 left-0 w-full h-full bg-neutral opacity-75 z-[5]"></div>

            // Content on top of the video and overlay
            <div class="relative z-[10] flex flex-col w-full h-full">
                // Navigation Bar
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
                <div class="flex flex-col justify-center items-center text-center flex-grow">
                    <h1 class="text-5xl text-white font-extrabold">{"Welcome to Our Website"}</h1>
                    <p class="text-2xl text-gray-200 mt-4">{"Innovative Solutions for Your Business"}</p>
                </div>
            </div>
        </div>
    }
}
