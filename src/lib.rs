mod components;
mod data;
mod pages;
mod views;

use yew::prelude::*;
use yew_router::prelude::*;

use pages::about_us::AboutUs;
use pages::contact::Contact;
use pages::home::Home;
use pages::not_found::NotFound;
use pages::products::Products;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/products")]
    Products,
    #[at("/about-us")]
    AboutUs,
    #[at("/contact")]
    Contact,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Products => html! { <Products /> },
        Route::AboutUs => html! { <AboutUs /> },
        Route::Contact => html! { <Contact /> },
        Route::NotFound => html! { <NotFound /> },
        _ => html! { <NotFound /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
