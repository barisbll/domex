use regex::Regex;
use web_sys::{window, HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

#[function_component(WriteUs)]
pub fn write_us() -> Html {
    let name = use_state(|| "".to_string());
    let email = use_state(|| "".to_string());
    let title = use_state(|| "".to_string());
    let message = use_state(|| "".to_string());
    let error = use_state(|| "".to_string()); // State to capture validation error

    fn is_valid_email(email: &str) -> bool {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        email_regex.is_match(email)
    }

    // Handler for the form submission
    let onsubmit = {
        let name = name.clone();
        let email = email.clone();
        let title = title.clone();
        let message = message.clone();
        let error = error.clone(); // Clone error state

        Callback::from(move |_| {
            // Clear previous error
            error.set("".to_string());

            // Validation checks
            if name.is_empty() || email.is_empty() || title.is_empty() || message.is_empty() {
                error.set("Wszystkie pola są wymagane!".to_string());
                return;
            }

            if !is_valid_email(&email) {
                error.set("Proszę wprowadzić prawidłowy adres e-mail.".to_string());
                return;
            }

            // Construct the mailto link if validation passes
            let mailto = format!(
                "mailto:robert-kiscinski@wp.pl?subject={}&body={}%0A%0A%0A%0AImię:%20{}%0AEmail:%20{}%0A",
                *title, *message, *name, *email
            );

            // Open the mail client
            if let Some(window) = window() {
                window.location().set_href(&mailto).unwrap();
            }

            // Reset form fields
            name.set("".to_string());
            email.set("".to_string());
            title.set("".to_string());
            message.set("".to_string());
        })
    };

    html! {
        <div class="relative w-full px-5 flex flex-col justify-center items-center xl:flex-row xl:justify-center gap-6">
            // Form card
            <div class="card card-compact bg-base-100 w-full md:w-3/4 xl:w-1/3">
                <div class="card-body">
                    <h2 class="card-title text-3xl text-center text-primary">{"Masz pytania? Napisz do nas!"}</h2>
                    <div class="form-control">
                        // Display validation error message if any
                        if !error.is_empty() {
                            <div class="alert alert-error mt-2">
                                <span>{ (*error).clone() }</span>
                            </div>
                        }

                        <label class="label mt-2">
                            <span class="label-text text-base-content">{"Imię"}</span>
                        </label>
                        <input
                            type="text"
                            placeholder="Twoje imię"
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
                            placeholder="Twój email"
                            class="input input-bordered"
                            value={(*email).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                email.set(input.value());
                            })}
                        />

                        <label class="label mt-2">
                            <span class="label-text text-base-content">{"Tytuł"}</span>
                        </label>
                        <input
                            type="text"
                            placeholder="Tytuł wiadomości"
                            class="input input-bordered"
                            value={(*title).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                title.set(input.value());
                            })}
                        />

                        <label class="label mt-2">
                            <span class="label-text text-base-content">{"Wiadomość"}</span>
                        </label>
                        <textarea
                            placeholder="Twoja wiadomość"
                            class="textarea textarea-bordered"
                            value={(*message).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let textarea: HtmlTextAreaElement = e.target_unchecked_into();
                                message.set(textarea.value());
                            })}
                        />
                        <button class="btn btn-primary mt-8" onclick={onsubmit}>{"Wyślij"}</button>
                    </div>
                </div>
                </div>
                // XL Text
                <div class="hidden xl:flex xl:justify-center xl:items-center xl:w-1/3">
                    <img src="static/img/email.webp" alt="Email Writing Person" class="w-3/4 brightness-75 " />
                </div>
            </div>
    }
}
