use crate::components::footer::Footer;
use crate::components::navbar::NavBar;
use crate::data::list_of_stal_doors::LIST_OF_STAL_DOORS;
use yew::prelude::*;

struct StalDoorImage {
    big_image_url: String,
    small_image_url: String,
}

#[function_component(StalDoors)]
pub fn stal_doors() -> Html {
    // Filter the products by the name
    let images: Vec<StalDoorImage> = LIST_OF_STAL_DOORS
        .iter()
        .map(|door| StalDoorImage {
            big_image_url: door.big_img_src.to_string(),
            small_image_url: door.small_img_src.to_string(),
        })
        .collect();

    // Set initial selected image to the first in the list
    let selected_image = use_state(|| images[0].big_image_url.clone());

    html! {
        <div class="min-h-screen lg:min-h-screen lg:flex lg:flex-col bg-base-300">
            <NavBar />
            <div class="container mx-auto flex flex-col lg:flex-row lg:justify-center lg:items-center lg:flex-1 mt-4 xl:mt-0 mb-12 xl:mb-0 py-8">
                <div class="flex-1 lg:flex-none flex justify-center items-center lg:w-1/3">
                    // Display selected big image
                    <img src={(*selected_image).clone()} class="max-h-screen md:w-1/3 lg:w-auto lg:h-full xl:w-3/4" />
                </div>
                <div class="w-full lg:w-1/2 md:mt-4 flex flex-row lg:grid lg:grid-cols-4 flex-wrap justify-center items-center content-start lg:content-center overflow-x-auto space-x-2 lg:space-x-0 p-2 lg:gap-4">
                    // Display list of small images
                    { for images.iter().map(|image| {
                        let on_click = {
                            let selected_image = selected_image.clone();
                            let big_image_url = image.big_image_url.clone();
                            Callback::from(move |_| selected_image.set(big_image_url.clone()))
                        };
                        html! {
                            <div onclick={on_click} class="cursor-pointer flex flex-col items-center w-1/4 md:w-1/5 lg:w-full md:pt-8">
                                <img src={image.small_image_url.clone()} class="w-24 h-auto lg:w-full" />
                            </div>
                        }
                    })}
                </div>
            </div>
            <Footer />
        </div>
    }
}
