use std::rc::Rc;

use shrt_common::errors::ServiceError;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

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

#[function_component]
pub fn UrlShortener() -> Html {
    let url_shortener_state = use_state(UrlShortenerState::default);
    let navigator = use_navigator().unwrap();

    let url = use_state(|| String::from("http://"));
    let link_name = use_state(String::default);

    let on_url_entry: Callback<String> = {
        let url = url.clone();

        Callback::from(move |value: String| {
            url.set(value.clone());
        })
    };

    let on_link_name_entry: Callback<String> = {
        let link_name = link_name.clone();

        Callback::from(move |value: String| {
            link_name.set(value);
        })
    };

    let on_link_name_debounce: Callback<String> = {
        let link_name = link_name.clone();
        let url_shortener_state = url_shortener_state.clone();

        Callback::from(move |value: String| {
            let link_name = link_name.clone();
            let url_shortener_state = url_shortener_state.clone();

            if value.is_empty() {
                url_shortener_state.set(UrlShortenerState::Initial);
                return;
            }

            wasm_bindgen_futures::spawn_local(async move {
                match ShrtApi::get_link_exists(&link_name).await {
                    Ok(link_exists) => {
                        url_shortener_state.set(if link_exists.exists {
                            UrlShortenerState::LinkExists
                        } else {
                            UrlShortenerState::Initial
                        });
                    }
                    Err(e) => {
                        url_shortener_state.set(UrlShortenerState::Error(e));
                    }
                }
            });
        })
    };

    let on_submit = {
        let url = url.clone();
        let link_name = link_name.clone();
        let url_shortener_state = url_shortener_state.clone();

        Callback::from(move |evt: SubmitEvent| {
            evt.prevent_default();

            let navigator = navigator.clone();
            let url_shortener_state = url_shortener_state.clone();
            let url = url.clone();
            let link_name = link_name.clone();

            url_shortener_state.set(UrlShortenerState::Loading);

            wasm_bindgen_futures::spawn_local(async move {
                match ShrtApi::shorten_url(&url, &link_name).await {
                    Ok(link) => {
                        navigator.push(&Route::Link { slug: link.slug });
                    }
                    Err(e) => {
                        url_shortener_state.set(UrlShortenerState::Error(e));
                    }
                }
            });
        })
    };

    let is_loading = (*url_shortener_state).is_loading();
    let is_link_exists = (*url_shortener_state).is_link_exists();
    let link_name_error = if is_link_exists {
        "Link already taken; please choose another or leave the field empty"
    } else {
        ""
    };
    html! {
        <>
            <form onsubmit={ on_submit }>
                <div class="mb-3">
                    <label for="url" class="form-label">{ "Target URL:" }</label>
                    <Input on_set_value={ on_url_entry.clone() } value={ (*url).clone() } disabled={ is_loading } required={ true } id="url" input_type="url" />
                </div>
                <div class="mb-3">
                    <label for="link-name" class="form-label">{ "Shortened Link:" }</label>
                    <div class="input-group mb-3">
                        <span class="input-group-text" id="basic-addon1">{ "https://shrt.rs/" }</span>
                        <Input on_set_value={ on_link_name_entry.clone() } on_debounce={ on_link_name_debounce.clone() } value={ (*link_name).clone() } disabled={ is_loading } placeholder="<random>" id="link-name" error={ link_name_error } />
                    </div>
                </div>

                <p class="text-center">
                    <button type="button" class="btn btn-outline-light" disabled={ is_loading } type="submit">
                        { "Shorten URL" }
                        if (*url_shortener_state).is_loading() {
                            <div class="spinner-border spinner-border-sm ms-2" role="status">
                                <span class="visually-hidden">{ "Loading..." }</span>
                            </div>
                        }
                    </button>
                </p>

                if let Some(error) = (*url_shortener_state).get_error() {
                    <ErrorAlert message={ "Could not shorten the URL" } error={Some(Rc::new(error.clone()))} />
                }
            </form>
        </>
    }
}
