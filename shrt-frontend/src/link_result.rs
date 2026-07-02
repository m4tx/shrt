use dioxus::prelude::*;
use shrt_common::errors::ServiceError;

use crate::api::ShrtApi;
use crate::error_alert::ErrorAlert;

#[derive(Clone, Debug)]
enum LinkResultState {
    Success { url: String },
    Error(ServiceError),
    Loading,
}

#[component]
pub fn LinkResult(slug: String) -> Element {
    let mut state: Signal<LinkResultState> = use_signal(|| LinkResultState::Loading);
    let mut slug_signal = use_signal(|| slug.clone());

    if *slug_signal.peek() != slug {
        slug_signal.set(slug.clone());
    }

    use_effect(move || {
        let s = slug_signal();
        if s.is_empty() {
            return;
        }
        state.set(LinkResultState::Loading);
        spawn(async move {
            match ShrtApi::get_link(&s).await {
                Ok(link) => state.set(LinkResultState::Success { url: link.url }),
                Err(e) => state.set(LinkResultState::Error(e)),
            }
        });
    });

    let shortened_url = format!("https://snip.rs/{}", urlencoding::encode(&slug));

    rsx! {
        div { class: "mb-3",
            p { class: "h1", "Shortened URL:" }
            a { href: shortened_url.clone(), class: "lead", {shortened_url.clone()} }
        }
        div { class: "mb-3 text-truncate placeholder-glow",
            p { class: "h2", "Target URL:" }
            match state.read().clone() {
                LinkResultState::Success { url } => rsx! {
                    a { href: url.clone(), {url.clone()} }
                },
                LinkResultState::Error(e) => rsx! {
                    ErrorAlert {
                        message: "Could not retrieve the target URL",
                        error: Some(e),
                    }
                },
                LinkResultState::Loading => rsx! {
                    span { class: "placeholder col-8" }
                },
            }
        }
    }
}
