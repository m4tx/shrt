use std::num::NonZeroU64;

use dioxus::prelude::*;
use rand::seq::IndexedRandom;
use shrt_common::errors::ServiceError;
use shrt_common::links::LinksResponse;

use crate::api::ShrtApi;
use crate::app::Route;
use crate::error_alert::ErrorAlert;
use crate::pagination::Pagination;
use crate::remove_link_modal::RemoveLinkModal;

#[derive(Clone, Debug)]
enum ListLinksState {
    Success(LinksResponse),
    Error(ServiceError),
    Loading,
}

impl ListLinksState {
    #[must_use]
    pub fn get_error(&self) -> Option<&ServiceError> {
        match self {
            ListLinksState::Error(e) => Some(e),
            _ => None,
        }
    }
}

#[component]
pub fn ListLinks(page: NonZeroU64) -> Element {
    let mut state: Signal<ListLinksState> = use_signal(|| ListLinksState::Loading);
    let mut removing_link_slug = use_signal(String::new);
    let mut iteration: Signal<u32> = use_signal(|| 0u32);
    let mut page_signal = use_signal(|| page);
    let navigator = use_navigator();

    if *page_signal.peek() != page {
        page_signal.set(page);
    }

    use_effect(move || {
        let p = page_signal();
        let _ = iteration();
        state.set(ListLinksState::Loading);
        spawn(async move {
            match ShrtApi::get_links(Some(p), None).await {
                Ok(r) => state.set(ListLinksState::Success(r)),
                Err(e) => state.set(ListLinksState::Error(e)),
            }
        });
    });

    let page_num = match &*state.read() {
        ListLinksState::Success(r) => NonZeroU64::new(r.num_pages),
        _ => None,
    };

    let error = state.read().get_error().cloned();

    rsx! {
        if let Some(e) = error {
            ErrorAlert { message: "Could not retrieve the list of links", error: Some(e) }
        } else {
            div { class: "table-responsive",
                table { class: "table table-striped table-hover",
                    thead {
                        tr {
                            th { scope: "col", "Slug" }
                            th { scope: "col", "URL" }
                            th { scope: "col", "Visits" }
                            th { scope: "col", "Created at" }
                            th { scope: "col", "Actions" }
                        }
                    }
                    tbody { class: "table-group-divider",
                        match state.read().clone() {
                            ListLinksState::Success(response) => rsx! {
                                for link in response.links {
                                    tr {
                                        td { class: "text-truncate", style: "max-width: 8rem;",
                                            a {
                                                href: format!(
                                                    "https://snip.rs/{}",
                                                    urlencoding::encode(&link.slug),
                                                ),
                                                "{link.slug}"
                                            }
                                        }
                                        td { class: "text-truncate", style: "max-width: 20rem;",
                                            a { href: link.url.clone(), "{link.url}" }
                                        }
                                        td { "{link.visits}" }
                                        td { "{format_date(link.created_at)}" }
                                        td { class: "pt-1 pb-1",
                                            button {
                                                onclick: move |_| removing_link_slug.set(link.slug.clone()),
                                                class: "btn btn-danger btn-sm",
                                                i { class: "bi bi-trash-fill" }
                                                " Remove"
                                            }
                                        }
                                    }
                                }
                            },
                            ListLinksState::Loading => rsx! {
                                for _ in 1..=10 {
                                    tr { class: "placeholder-glow",
                                        td { span { class: gen_random_col_class() } }
                                        td { span { class: gen_random_col_class() } }
                                        td { span { class: gen_random_col_class() } }
                                        td { span { class: "placeholder col-8" } }
                                        td { span { class: "placeholder col-8" } }
                                    }
                                }
                            },
                            ListLinksState::Error(_) => rsx! {},
                        }
                    }
                }
            }

            Pagination {
                current_page: page,
                page_num: page_num,
                on_set_value: move |p: NonZeroU64| {
                    navigator.push(Route::ListLinks { page: p });
                },
            }

            RemoveLinkModal {
                slug: removing_link_slug.read().clone(),
                on_remove: move |_| *iteration.write() += 1,
            }
        }
    }
}

fn gen_random_col_class() -> String {
    let cols = ["3", "4", "5", "6", "7", "8", "10", "12"];
    let col = cols.choose(&mut rand::rng()).unwrap();
    format!("placeholder col-{col}")
}

fn format_date(datetime: chrono::DateTime<chrono::Utc>) -> String {
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}
