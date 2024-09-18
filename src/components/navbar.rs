use yew::prelude::*;

#[function_component(NavBar)]
pub fn navbar() -> Html {
    let menu_visible = use_state(|| false);

    let toggle_menu = {
        let menu_visible = menu_visible.clone();
        Callback::from(move |_| menu_visible.set(!*menu_visible))
    };

    html! {
        <nav class="navbar bg-base-100 w-full py-4 bg-transparent sticky top-0 z-[50]">
            <div class="navbar-start pl-8">
                <a href="/" class="btn btn-ghost text-2xl text-base-content">{"DOMEX"}</a>
            </div>

            <div class="navbar-end pr-8">
                // Mobile hamburger menu button using DaisyUI dropdown
                <div class="dropdown dropdown-end lg:hidden">
                <label tabindex="0" class="btn btn-square btn-ghost text-base-content" onclick={toggle_menu.clone()}>
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
                        </svg>
                    </label>

                    // Dropdown menu for mobile
                    {
                        if *menu_visible {
                            html! {
                            <ul tabindex="0" class="menu dropdown-content z-[1] p-2 shadow bg-base-100 rounded-box w-52">
                                <li><a href="/" class="text-xl font-bold text-base-content">{"Strona Główna"}</a></li>
                                <li><a href="/products" class="text-xl font-bold text-base-content">{"Produkty"}</a></li>
                                <li><a href="/installations" class="text-xl font-bold text-base-content">{"Realizacja"}</a></li>
                                <li><a href="/about-us" class="text-xl font-bold text-base-content">{"O Nas"}</a></li>
                                <li><a href="/contact" class="text-xl font-bold text-base-content">{"Kontakt"}</a></li>
                            </ul>
                            }
                        } else {
                            html! {}
                        }
                    }

                </div>

                // Desktop menu items
                <ul id="menu-items" class="hidden lg:flex lg:gap-5">
                <li><a href="/" class="text-xl font-bold text-base-content">{"Strona Główna"}</a></li>
                <li><a href="/products" class="text-xl font-bold text-base-content">{"Produkty"}</a></li>
                <li><a href="/installations" class="text-xl font-bold text-base-content">{"Realizacja"}</a></li>
                <li><a href="/about-us" class="text-xl font-bold text-base-content">{"O Nas"}</a></li>
                <li><a href="/contact" class="text-xl font-bold text-base-content">{"Kontakt"}</a></li>
                </ul>
            </div>
        </nav>
    }
}
