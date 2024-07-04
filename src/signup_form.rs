use dioxus::prelude::*;

pub fn SignupForm() -> Element {
    rsx! {
        form {
            display: "flex",
            flex_direction: "column",

            onsubmit: onsubmit,

            div {
                label {
                    "Enter your email: "
                }
                input {
                    r#type: "text",
                    placeholder: "email",
                }
            }

            div {
                label {
                    "First Name: "
                }
                input {
                    r#type: "text",
                    placeholder: "First Name",
                }
            }

            div {
                label {
                    "Last Name: "
                }
                input {
                    r#type: "text",
                    placeholder: "Last Name",
                }
            }

            button {
                r#type: "submit",

                "Submit!"
            }
        }
    }
}


fn handle_input(event: Event<FormData>) {
    web_sys::window().unwrap().alert_with_message(&event.value());
}

fn onsubmit(event: Event<FormData>) {
    web_sys::window().unwrap().alert_with_message(&format!("{:?}", &event));
}