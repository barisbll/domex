mod components;
mod data;
mod pages;
mod views;

use yew::prelude::*;
use yew_router::prelude::*;

use pages::about_us::AboutUs;
use pages::contact::Contact;
use pages::home::Home;
use pages::inside_door::InsideDoor;
use pages::inside_doors::InsideDoors;
use pages::installations::Installations;
use pages::not_found::NotFound;
use pages::room_doors::RoomDoors;
use pages::stal_doors::StalDoors;
use pages::steel_doors::SteelDoors;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/inside-doors")]
    InsideDoors,
    #[at("/installations")]
    Installations,
    #[at("/inside-doors/:id")]
    InsideDoor { id: String },
    #[at("/room-doors")]
    RoomDoors,
    #[at("/steel-doors")]
    SteelDoors,
    #[at("/stal-doors")]
    StalDoors,
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
        Route::InsideDoors => html! { <InsideDoors /> },
        Route::RoomDoors => html! { <RoomDoors /> },
        Route::SteelDoors => html! { <SteelDoors /> },
        Route::StalDoors => html! { <StalDoors /> },
        Route::Installations => html! { <Installations /> },
        Route::InsideDoor { id } => html! { <InsideDoor id={id} /> },
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
