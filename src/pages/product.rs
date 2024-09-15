use crate::components::footer::Footer;
use crate::components::navbar::NavBar;
use crate::data::list_of_products::LIST_OF_PRODUCTS;
use yew::prelude::*;

fn extract_base_url(text: String) -> String {
    let mut parts: Vec<&str> = text.split('/').collect();
    parts.pop();

    format!("{}/", parts.join("/"))
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

struct ProductImage {
    display_name: String,
    small_image_url: String,
    big_image_url: String,
}

#[function_component(Product)]
pub fn product(props: &Props) -> Html {
    let product_href = format!("/products/{}/", props.id);
    let product = LIST_OF_PRODUCTS.iter().find(|&p| p.href == product_href);

    let images: Option<Vec<ProductImage>> = product.map(|product| {
        let variants = vec![
            ("Biały", ""),
            ("Dąb Złoty", "dabzloty"),
            ("Jesion", "jesion"),
            ("Olcha", "olcha"),
            ("Orzech", "orzech"),
            ("Orzech Ciemny", "orzechciemny"),
            ("Wenge", "wenge"),
            ("Wiśnia", "wisnia"),
        ];

        let base_url = extract_base_url(product.img_src.to_string().clone());

        variants
            .iter()
            .map(|(display_name, type_suffix)| {
                let suffix = if type_suffix.is_empty() {
                    "".to_string()
                } else {
                    format!("-{}", type_suffix)
                };
                let small_image_url =
                    format!("{}{}{}-200x418.png", base_url, product.base_name, suffix);
                let big_image_url =
                    format!("{}{}{}-431x900.png", base_url, product.base_name, suffix);

                ProductImage {
                    display_name: display_name.to_string(),
                    small_image_url,
                    big_image_url,
                }
            })
            .collect()
    });

    let selected_image = use_state(|| images.as_ref().unwrap()[0].big_image_url.clone());

    html! {
        <div class="min-h-screen lg:h-screen lg:flex lg:flex-col bg-base-300">
            <NavBar />
            {
                if let Some(product) = product {
                    let images = images.unwrap();

                    html! {
                        <div class="container mx-auto flex flex-col lg:flex-row lg:justify-center lg:items-center lg:flex-1 mt-4 xl:mt-0 mb-12 xl:mb-0  ">
                            <div class="flex-1 lg:flex-none flex justify-center items-center lg:w-1/3">
                                <img src={(*selected_image).clone()} alt={product.name} class="max-h-screen md:w-1/3 lg:w-auto lg:h-full xl:w-3/4 " />
                            </div>
                            <div class="w-full lg:w-1/2 md:mt-4 flex flex-row lg:grid lg:grid-cols-4 flex-wrap justify-center items-center content-start lg:content-center overflow-x-auto lg:overflow-y-auto space-x-2 lg:space-x-0 p-2">
                                { for images.iter().map(|image| {
                                    let on_click = {
                                        let selected_image = selected_image.clone();
                                        let big_image_url = image.big_image_url.clone();
                                        Callback::from(move |_| selected_image.set(big_image_url.clone()))
                                    };
                                    html! {
                                        <div onclick={on_click} class="cursor-pointer flex flex-col items-center w-1/4 md:w-1/5 lg:w-full md:mt-4 lg:mt-0">
                                            <img src={image.small_image_url.clone()} alt={image.display_name.clone()} class="w-24 h-auto lg:w-full" />
                                            <p class="text-center mt-1 lg:mt-0 text-sm">{ image.display_name.clone() }</p>
                                        </div>
                                    }
                                })}
                            </div>
                        </div>
                    }
                } else {
                    html! {
                        <div class="flex justify-center items-center h-full">
                            <p class="text-2xl text-red-500">{ "Product not found." }</p>
                        </div>
                    }
                }
            }
            <Footer />
        </div>
    }
}
