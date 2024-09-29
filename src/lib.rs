mod components;
mod data;
mod pages;
mod views;

use yew::prelude::*;
use yew_router::prelude::*;

use pages::about_us::AboutUs;
use pages::contact::Contact;
use pages::home::Home;
use pages::installations::Installations;
use pages::not_found::NotFound;
use pages::product::Product;
use pages::products::Products;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/domex/")]
    Home,
    #[at("/domex/products")]
    Products,
    #[at("/domex/installations")]
    Installations,
    #[at("/domex/products/:id")]
    Product { id: String },
    #[at("/domex/about-us")]
    AboutUs,
    #[at("/domex/contact")]
    Contact,
    #[not_found]
    #[at("/domex/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Products => html! { <Products /> },
        Route::Installations => html! { <Installations /> },
        Route::Product { id } => html! { <Product id={id} /> },
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
