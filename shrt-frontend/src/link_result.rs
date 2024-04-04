use std::rc::Rc;

use shrt_common::errors::ServiceError;
use yew::prelude::*;

use crate::api::ShrtApi;
use crate::error_alert::ErrorAlert;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub slug: AttrValue,
}

#[derive(Clone, Debug, Default)]
pub enum LinkResultState {
    Success {
        slug: String,
        url: String,
    },
    Error(ServiceError),
    #[default]
    Loading,
}

#[function_component]
pub fn LinkResult(props: &Props) -> Html {
    let Props { slug } = props;

    let link_result_state = use_state(LinkResultState::default);
    {
        let link_result_state = link_result_state.clone();
        let slug = slug.clone();

        use_effect_with(slug.clone(), move |_| {
            if slug.is_empty() {
                return;
            }

            wasm_bindgen_futures::spawn_local(async move {
                match ShrtApi::get_link(&slug).await {
                    Ok(link) => {
                        link_result_state.set(LinkResultState::Success {
                            slug: link.slug,
                            url: link.url,
                        });
                    }
                    Err(e) => {
                        link_result_state.set(LinkResultState::Error(e));
                    }
                }
            });
        });
    }

    // TODO url encode?
    let shortened_url = format!("https://shrt.rs/{}", urlencoding::encode(slug));
    // TODO replace shrt.rs with an actual config-driven variable
    let target_url_element = match (*link_result_state).clone() {
        LinkResultState::Success { url, .. } => {
            html! {
                <a href={url.clone()}>{url}</a>
            }
        }
        LinkResultState::Error(e) => {
            html! {
                <ErrorAlert message={ "Could not retrieve the target URL" } error={Some(Rc::new(e))} />
            }
        }
        LinkResultState::Loading => {
            html! {
                <span class="placeholder col-8"></span>
            }
        }
    };

    html! {
        <>
            <div class="mb-3">
                <p class="h1">{ "Shortened URL:" }</p>
                <a href={shortened_url.clone()} class="lead">{shortened_url}</a>
            </div>
            <div class="mb-3 text-truncate placeholder-glow">
                <p class="h2">{ "Target URL:" }</p>
                {target_url_element}
            </div>
        </>
    }
}
