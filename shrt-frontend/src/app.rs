use std::num::NonZeroU64;

use dioxus::prelude::*;

use crate::link_result::LinkResult;
use crate::list_links::ListLinks;
use crate::not_found::NotFound;
use crate::url_shortener::UrlShortener;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
    #[route("/app/link/:slug")]
    LinkResult { slug: String },
    #[route("/app/links/:page")]
    ListLinks { page: NonZeroU64 },
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

#[component]
fn Layout() -> Element {
    rsx! {
        nav { class: "navbar navbar-expand-md navbar-dark bg-dark mb-4",
            div { class: "container",
                Link { to: Route::Home {}, class: "navbar-brand", "shrt" }
                button {
                    class: "navbar-toggler",
                    r#type: "button",
                    "data-bs-toggle": "collapse",
                    "data-bs-target": "#navbarCollapse",
                    "aria-controls": "navbarCollapse",
                    "aria-expanded": "false",
                    "aria-label": "Toggle navigation",
                    span { class: "navbar-toggler-icon" }
                }
                div { class: "collapse navbar-collapse", id: "navbarCollapse",
                    ul { class: "navbar-nav ms-auto mb-2 mb-md-0",
                        li { class: "nav-item",
                            a {
                                class: "nav-link",
                                href: "https://github.com/m4tx/shrt",
                                "Source"
                            }
                        }
                    }
                }
            }
        }
        main { class: "container",
            div { class: "bg-body-tertiary p-5 rounded",
                h1 { "shrt" }
                Outlet::<Route> {}
            }
        }
    }
}

#[component]
pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! { UrlShortener {} }
}
