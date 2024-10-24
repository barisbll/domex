use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

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
                // Using Link from yew_router instead of <a> for client-side navigation
                <Link<Route> to={Route::Home} classes="btn btn-ghost text-2xl text-base-content">{"DOMEX"}</Link<Route>>
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
                                <li><Link<Route> to={Route::Home} classes="text-xl font-bold text-base-content">{"Strona Główna"}</Link<Route>></li>
                                <li><Link<Route> to={Route::InsideDoors} classes="text-xl font-bold text-base-content">{"Drzwi Wewnątrzklatkowe"}</Link<Route>></li>
                                <li><Link<Route> to={Route::Installations} classes="text-xl font-bold text-base-content">{"Realizacja"}</Link<Route>></li>
                                <li><Link<Route> to={Route::AboutUs} classes="text-xl font-bold text-base-content">{"O Nas"}</Link<Route>></li>
                                <li><Link<Route> to={Route::Contact} classes="text-xl font-bold text-base-content">{"Kontakt"}</Link<Route>></li>
                            </ul>
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>

                // Desktop menu items
                <ul id="menu-items" class="hidden lg:flex lg:gap-5">
                    <li><Link<Route> to={Route::Home} classes="text-xl font-bold text-base-content">{"Strona Główna"}</Link<Route>></li>
                    <li><Link<Route> to={Route::InsideDoors} classes="text-xl font-bold text-base-content">{"Drzwi Wewnątrzklatkowe"}</Link<Route>></li>
                    <li><Link<Route> to={Route::Installations} classes="text-xl font-bold text-base-content">{"Realizacja"}</Link<Route>></li>
                    <li><Link<Route> to={Route::AboutUs} classes="text-xl font-bold text-base-content">{"O Nas"}</Link<Route>></li>
                    <li><Link<Route> to={Route::Contact} classes="text-xl font-bold text-base-content">{"Kontakt"}</Link<Route>></li>
                </ul>
            </div>
        </nav>
    }
}
