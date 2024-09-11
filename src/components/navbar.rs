use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlInputElement};
use yew::prelude::*;

#[function_component(NavBar)]
pub fn navbar() -> Html {
    let menu_visible = use_state(|| false);

    // Use effect to toggle the menu based on the state of the checkbox
    use_effect_with(menu_visible.clone(), move |menu_visible| {
        // Get the document
        let document = gloo::utils::document();

        // Get the checkbox element by its id and cast to HtmlInputElement
        let menu_toggle = document
            .get_element_by_id("menu-toggle")
            .unwrap()
            .unchecked_into::<HtmlInputElement>();

        // Get the mobile menu element by its id
        let mobile_menu = document.get_element_by_id("mobile-menu").unwrap();

        if **menu_visible {
            // If the menu should be visible, check the checkbox and display the mobile menu
            menu_toggle.set_checked(true);
            mobile_menu.set_class_name("lg:hidden absolute top-16 left-0 w-full bg-base-100 p-4");
        // Remove the 'hidden' class
        } else {
            // If the menu should be hidden, uncheck the checkbox and hide the mobile menu
            menu_toggle.set_checked(false);
            mobile_menu
                .set_class_name("lg:hidden absolute top-16 left-0 w-full bg-base-100 p-4 hidden");
            // Add the 'hidden' class
        }

        // No cleanup needed, return ()
        ()
    });

    let toggle_menu = {
        let menu_visible = menu_visible.clone();
        Callback::from(move |_| menu_visible.set(!*menu_visible))
    };

    html! {
        <nav class="navbar bg-base-100 w-full py-4 bg-transparent sticky top-0 z-[10]">
            <div class="navbar-start pl-8">
                <a href="/" class="btn btn-ghost text-2xl">{"DOMEX"}</a>
            </div>
            <div class="navbar-end pr-8">
                // Mobile hamburger menu button
                <div class="lg:hidden">
                    <label for="menu-toggle" class="btn btn-square btn-ghost text-white" onclick={toggle_menu.clone()}>
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="w-6 h-6">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16m-7 6h7" />
                        </svg>
                    </label>
                </div>

                // Menu items
                <ul id="menu-items" class="hidden lg:flex lg:gap-5">
                    <li><a href="/" class="text-white text-xl font-bold">{"Strona Główna"}</a></li>
                    <li><a href="/products" class="text-white text-xl font-bold">{"Produkty"}</a></li>
                    <li><a href="/about-us" class="text-white text-xl font-bold">{"O Nas"}</a></li>
                    <li><a href="/contact" class="text-white text-xl font-bold">{"Kontakt"}</a></li>
                </ul>
            </div>

            // Mobile menu for small screens
            <input type="checkbox" id="menu-toggle" class="hidden" />
            <div class="lg:hidden absolute top-16 left-0 w-full bg-base-100 p-4 hidden" id="mobile-menu">
                <ul class="flex flex-col gap-4">
                    <li><a href="/" class="text-xl font-bold">{"Strona Główna"}</a></li>
                    <li><a href="/products" class="text-xl font-bold">{"Produkty"}</a></li>
                    <li><a href="/about-us" class="text-xl font-bold">{"O Nas"}</a></li>
                    <li><a href="/contact" class="text-xl font-bold">{"Kontakt"}</a></li>
                </ul>
            </div>
        </nav>
    }
}
