use yew::prelude::*;
use yew_router::prelude::Link;

use crate::Route;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
    <footer class="footer bg-base-200 text-base-content p-10">
        <aside>
        <Link<Route> to={Route::Home} classes="btn btn-ghost text-2xl text-base-content">{"DOMEX"}</Link<Route>>
      </aside>
      <nav>
            <h6 class="footer-title">{"Produkty"}</h6>
            <p class="link link-hover"><Link<Route> to={Route::Home}>{"Strona Główna"}</Link<Route>></p>
            <p class="link link-hover"><Link<Route> to={Route::InsideDoors}>{"Drzwi Wewnątrzklatkowe"}</Link<Route>></p>
            <p class="link link-hover"><Link<Route> to={Route::RoomDoors}>{"Drzwi Pokojowe"}</Link<Route>></p>
            <p class="link link-hover"><Link<Route> to={Route::SteelDoors}>{"Drzwi Stalowe"}</Link<Route>></p>

      </nav>
      <nav>
            <h6 class="footer-title">{"Produkty"}</h6>
            <p class="link link-hover"><Link<Route> to={Route::StalDoors}>{"Drzwi Wewnątrzklatkowe z Listwą"}</Link<Route>></p>
            <p class="link link-hover"><Link<Route> to={Route::Installations}>{"Realizacja"}</Link<Route>></p>
            <p class="link link-hover"><Link<Route> to={Route::AboutUs}>{"O Nas"}</Link<Route>></p>
            <p class="link link-hover"><Link<Route> to={Route::Contact}>{"Kontakt"}</Link<Route>></p>
      </nav>
      <nav>
            <h6 class="footer-title">{"Nasi partnerzy"}</h6>
                <a href="https://www.artusdrzwi.pl/" target="_blank" class="pb-1">
                    <img src="static/img/partners/artusdrzwi.png" alt="Artusdrzwi logo" class="w-24 h-7" />
                </a>
                <a href="https://futryna.com.pl/" target="_blank"  class="pb-1">
                    <img src="static/img/partners/stalprodukt.svg" alt="Stalprodukt logo" class="w-48" />
                </a>
                <a href="https://sendeckidrzwi.pl/" target="_blank" >
                    <img src="static/img/partners/sendeckidrzwi.jpg" alt="sendeckidrzwi logo" class="w-32" />
                </a>

      </nav>

    </footer>
        }
}
