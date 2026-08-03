use std::num::NonZeroU64;

use dioxus::prelude::*;
use shrt_common::config::AppConfig;
use shrt_common::errors::ServiceError;

use crate::api::ShrtApi;
use crate::error_alert::ErrorAlert;
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
    let config = use_context::<Signal<AppConfig>>();
    let app_name = config.read().app_name.clone();

    rsx! {
        nav { class: "navbar navbar-expand-md navbar-dark bg-dark mb-4",
            div { class: "container",
                Link { to: Route::Home {}, class: "navbar-brand", {app_name.clone()} }
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
                h1 { {app_name} }
                Outlet::<Route> {}
            }
        }
    }
}

#[component]
pub fn App() -> Element {
    let mut config_state: Signal<Option<Result<AppConfig, ServiceError>>> = use_signal(|| None);

    use_effect(move || {
        spawn(async move {
            config_state.set(Some(ShrtApi::get_config().await));
        });
    });

    match config_state.read().clone() {
        None => rsx! {
            div { class: "d-flex justify-content-center align-items-center vh-100",
                div { class: "spinner-border", role: "status",
                    span { class: "visually-hidden", "Loading..." }
                }
            }
        },
        Some(Err(e)) => rsx! {
            div { class: "container mt-5",
                ErrorAlert {
                    message: "Failed to load application configuration",
                    error: Some(e),
                }
            }
        },
        Some(Ok(config)) => rsx! {
            AppRoot { config }
        },
    }
}

#[component]
fn AppRoot(config: AppConfig) -> Element {
    use_context_provider(|| Signal::new(config.clone()));
    rsx! { Router::<Route> {} }
}

#[component]
fn Home() -> Element {
    rsx! { UrlShortener {} }
}
