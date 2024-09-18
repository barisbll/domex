use yew::prelude::*;

use crate::components::navbar::NavBar;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <>
        <div class="bg-base-300 ">
        <NavBar />
            <header class="h-100  min-h-screen text-center py-4 text-white flex justify-center items-center">
                <h1 class="text-3xl font-bold text-base-content">{"404 - Nie znaleziono strony"}</h1>
            </header>
        </div>
        </>
    }
}
