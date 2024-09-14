// product.rs
use yew::prelude::*;

use crate::components::navbar::NavBar;
use crate::data::list_of_products::{Product as ProductData, LIST_OF_PRODUCTS};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

#[function_component(Product)]
pub fn product(props: &Props) -> Html {
    let product_href = format!("/products/{}/", props.id);

    // Find the product in the list where the href matches
    let product = LIST_OF_PRODUCTS.iter().find(|&p| p.href == product_href);

    html! {
        <div class="min-h-screen bg-base-300">
            <NavBar />
            {
                if let Some(product) = product {
                    html! {
                        <>
                            <h1>{ product.name.clone() }</h1>
                            <img src={product.img_src} alt={product.name} />
                            // Include other product details as needed
                        </>
                    }
                } else {
                    html! {
                        <p>{ "Product not found." }</p>
                    }
                }
            }
        </div>
    }
}
