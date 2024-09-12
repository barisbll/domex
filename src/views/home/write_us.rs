use web_sys::{window, HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

#[function_component(WriteUs)]
pub fn write_us() -> Html {
    // State hooks to capture form input
    let name = use_state(|| "".to_string());
    let email = use_state(|| "".to_string());
    let title = use_state(|| "".to_string());
    let message = use_state(|| "".to_string());

    // Handler for the form submission
    let onsubmit = {
        let name = name.clone();
        let email = email.clone();
        let title = title.clone();
        let message = message.clone();

        Callback::from(move |_| {
            // Construct the mailto link
            let mailto = format!(
                "mailto:support@example.com?subject={}&body={}%0A%0A%0A%0AName:%20{}%0AEmail:%20{}%0A",
                *title, *message, *name, *email
            );

            // Open the mail client
            if let Some(window) = window() {
                window.location().set_href(&mailto).unwrap();
            }

            name.set("".to_string());
            email.set("".to_string());
            title.set("".to_string());
            message.set("".to_string());
        })
    };

    // Todo: Validate the inputs before sending the email
    // Todo: Read the tutorial

    html! {
        <div class="relative w-full h-screen px-5 flex flex-col justify-center items-center">
            <div class="card card-compact bg-base-100 w-full">
                <div class="card-body">
                    <h2 class="card-title text-3xl text-center text-primary">{"Write to us"}</h2>
                    <div class="form-control">
                        <label class="label mt-2">
                            <span class="label-text text-base-content">{"Name"}</span>
                        </label>
                        <input
                            type="text"
                            placeholder="Your name"
                            class="input input-bordered"
                            value={(*name).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                name.set(input.value());
                            })}
                        />

                        <label class="label mt-2">
                            <span class="label-text text-base-content">{"Email"}</span>
                        </label>
                        <input
                            type="email"
                            placeholder="Your email"
                            class="input input-bordered"
                            value={(*email).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                email.set(input.value());
                            })}
                        />

                        <label class="label mt-2">
                            <span class="label-text text-base-content">{"Title"}</span>
                        </label>
                        <input
                            type="text"
                            placeholder="Message title"
                            class="input input-bordered"
                            value={(*title).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                title.set(input.value());
                            })}
                        />

                        <label class="label mt-2">
                            <span class="label-text text-base-content">{"Message"}</span>
                        </label>
                        <textarea
                            placeholder="Your message"
                            class="textarea textarea-bordered"
                            value={(*message).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let textarea: HtmlTextAreaElement = e.target_unchecked_into();
                                message.set(textarea.value());
                            })}
                        />
                        <button class="btn btn-primary mt-8" onclick={onsubmit}>{"Submit"}</button>
                    </div>
                </div>
            </div>
        </div>
    }
}
