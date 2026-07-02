use dioxus::prelude::*;

use crate::app::Route;

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        div { class: "mb-3",
            h1 { class: "display-2", "Page not found" }
            p { class: "lead", "The page you have requested could not be found." }
            Link { to: Route::Home {}, class: "btn btn-primary",
                i { class: "bi bi-house-fill" }
                " Go to homepage"
            }
        }
    }
}
