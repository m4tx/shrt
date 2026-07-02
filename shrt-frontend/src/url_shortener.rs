use dioxus::prelude::*;
use shrt_common::errors::ServiceError;

use crate::api::ShrtApi;
use crate::app::Route;
use crate::error_alert::ErrorAlert;
use crate::input::Input;

#[derive(Clone, Debug, Default)]
pub enum UrlShortenerState {
    Error(ServiceError),
    LinkExists,
    Loading,
    #[default]
    Initial,
}

impl UrlShortenerState {
    #[must_use]
    pub fn is_loading(&self) -> bool {
        matches!(self, UrlShortenerState::Loading)
    }

    #[must_use]
    pub fn is_link_exists(&self) -> bool {
        matches!(self, UrlShortenerState::LinkExists)
    }

    #[must_use]
    pub fn get_error(&self) -> Option<&ServiceError> {
        match self {
            UrlShortenerState::Error(e) => Some(e),
            _ => None,
        }
    }
}

#[component]
pub fn UrlShortener() -> Element {
    let mut url = use_signal(|| "http://".to_string());
    let mut link_name = use_signal(String::new);
    let mut state: Signal<UrlShortenerState> = use_signal(UrlShortenerState::default);
    let navigator = use_navigator();

    let is_loading = state.read().is_loading();
    let is_link_exists = state.read().is_link_exists();
    let error = state.read().get_error().cloned();
    let link_name_error = if is_link_exists {
        "Link already taken; please choose another or leave the field empty"
    } else {
        ""
    };

    rsx! {
        form {
            onsubmit: move |evt| {
                evt.prevent_default();
                let url_val = url.read().clone();
                let link_name_val = link_name.read().clone();
                state.set(UrlShortenerState::Loading);
                spawn(async move {
                    match ShrtApi::shorten_url(&url_val, &link_name_val).await {
                        Ok(link) => {
                            navigator.push(Route::LinkResult { slug: link.slug });
                        }
                        Err(e) => {
                            state.set(UrlShortenerState::Error(e));
                        }
                    }
                });
            },
            div { class: "mb-3",
                label { r#for: "url", class: "form-label", "Target URL:" }
                Input {
                    on_set_value: move |v| url.set(v),
                    value: url.read().clone(),
                    disabled: is_loading,
                    required: true,
                    id: "url",
                    input_type: "url",
                }
            }
            div { class: "mb-3",
                label { r#for: "link-name", class: "form-label", "Shortened Link:" }
                div { class: "input-group mb-3",
                    span { class: "input-group-text", id: "basic-addon1", "https://snip.rs/" }
                    Input {
                        on_set_value: move |v| link_name.set(v),
                        on_debounce: move |v: String| {
                            if v.is_empty() {
                                state.set(UrlShortenerState::Initial);
                                return;
                            }
                            spawn(async move {
                                match ShrtApi::get_link_exists(&v).await {
                                    Ok(r) => state.set(if r.exists {
                                        UrlShortenerState::LinkExists
                                    } else {
                                        UrlShortenerState::Initial
                                    }),
                                    Err(e) => state.set(UrlShortenerState::Error(e)),
                                }
                            });
                        },
                        value: link_name.read().clone(),
                        disabled: is_loading,
                        placeholder: "<random>",
                        id: "link-name",
                        error: link_name_error.to_string(),
                    }
                }
            }
            p { class: "text-center",
                button {
                    r#type: "submit",
                    class: "btn btn-outline-light",
                    disabled: is_loading,
                    "Shorten URL"
                    if is_loading {
                        div { class: "spinner-border spinner-border-sm ms-2", role: "status",
                            span { class: "visually-hidden", "Loading..." }
                        }
                    }
                }
            }
            if let Some(e) = error {
                ErrorAlert { message: "Could not shorten the URL", error: Some(e) }
            }
        }
    }
}
