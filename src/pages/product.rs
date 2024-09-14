use crate::components::navbar::NavBar;
use crate::data::list_of_products::LIST_OF_PRODUCTS;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

#[function_component(Product)]
pub fn product(props: &Props) -> Html {
    // Construct the product href using the id from props
    let product_href = format!("/products/{}/", props.id);

    // Find the product in the list where the href matches
    let product = LIST_OF_PRODUCTS.iter().find(|&p| p.href == product_href);

    // Define selected_image state
    let selected_image = use_state(|| "".to_string());

    // Struct to hold image data
    struct ProductImage {
        display_name: String,
        small_image_url: String,
        big_image_url: String,
    }

    // Define images as an Option<Vec<ProductImage>>
    let images: Option<Vec<ProductImage>> = product.map(|product| {
        // Define the variants and their display names
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

        // Extract the base name from the product's base_name
        let base_name = product.base_name;

        // Base URL for images
        let base_url = "https://sendeckidrzwi.pl/wp-content/uploads/2015/10/";

        // Generate the list of images
        variants
            .iter()
            .map(|(display_name, type_suffix)| {
                let suffix = if type_suffix.is_empty() {
                    "".to_string()
                } else {
                    format!("-{}", type_suffix)
                };
                let small_image_url = format!("{}{}{}-200x418.png", base_url, base_name, suffix);
                let big_image_url = format!("{}{}{}-431x900.png", base_url, base_name, suffix);

                ProductImage {
                    display_name: display_name.to_string(),
                    small_image_url,
                    big_image_url,
                }
            })
            .collect()
    });

    // Use an effect to set selected_image when images change
    // {
    //     let selected_image = selected_image.clone();
    //     if let Some(images) = &images {
    //         let first_image_url = images[0].big_image_url.clone();
    //         selected_image.set(first_image_url);
    //     }
    // }

    html! {
        <div class="min-h-screen bg-base-300">
            <NavBar />
            {
                if let Some(product) = product {
                    let images = images.unwrap();

                    html! {
                        <div class="container mx-auto flex flex-col lg:flex-row mt-4">
                            <div class="flex-1 flex justify-center items-center">
                                <img src={(*selected_image).clone()} alt={product.name} class="max-h-screen" />
                            </div>
                            <div class="w-full lg:w-1/4 flex flex-row lg:flex-col overflow-x-auto lg:overflow-y-auto space-x-2 lg:space-x-0 lg:space-y-2 p-2">
                                { for images.iter().map(|image| {
                                    let on_click = {
                                        let selected_image = selected_image.clone();
                                        let big_image_url = image.big_image_url.clone();
                                        Callback::from(move |_| selected_image.set(big_image_url.clone()))
                                    };
                                    html! {
                                        <div onclick={on_click} class="cursor-pointer flex flex-col items-center">
                                            <img src={image.small_image_url.clone()} alt={image.display_name.clone()} class="w-24 h-auto lg:w-full" />
                                            <p class="text-center mt-1 text-sm">{ image.display_name.clone() }</p>
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
        </div>
    }
}
