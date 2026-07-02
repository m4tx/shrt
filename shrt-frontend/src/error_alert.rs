use dioxus::prelude::*;
use shrt_common::errors::ServiceError;

#[component]
pub fn ErrorAlert(
    #[props(default)] message: String,
    #[props(default)] error: Option<ServiceError>,
) -> Element {
    let displayed_message = {
        let mut s = String::new();

        if !message.is_empty() {
            s.push_str(&message);
        }
        if let Some(err) = &error {
            if !s.is_empty() {
                s.push_str(": ");
            }
            s.push_str(&err.error);
            if let Some(err_msg) = &err.message {
                s.push_str(": ");
                s.push_str(err_msg);
            }
        }

        s
    };

    rsx! {
        div { class: "alert alert-danger d-flex align-items-center", role: "alert",
            i { class: "bi bi-exclamation-triangle-fill flex-shrink-0 me-2" }
            div { {displayed_message} }
        }
    }
}
