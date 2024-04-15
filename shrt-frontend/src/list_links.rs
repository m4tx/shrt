use std::num::NonZeroU64;
use std::rc::Rc;

use implicit_clone::unsync::IString;
use rand::seq::SliceRandom;
use shrt_common::errors::ServiceError;
use shrt_common::links::LinksResponse;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::api::ShrtApi;
use crate::app::Route;
use crate::error_alert::ErrorAlert;
use crate::pagination::Pagination;
use crate::remove_link_modal::RemoveLinkModal;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or(NonZeroU64::new(1).unwrap())]
    pub page: NonZeroU64,
}

#[derive(Clone, Debug, Default)]
pub enum ListLinksState {
    Success(LinksResponse),
    Error(ServiceError),
    #[default]
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

#[function_component]
pub fn ListLinks(props: &Props) -> Html {
    let Props { page } = props;
    let navigator = use_navigator().unwrap();

    let list_links_state = use_state(ListLinksState::default);
    let removing_link_slug = use_state(IString::default);
    let iteration: UseStateHandle<u32> = use_state(|| 0);

    {
        let link_result_state = list_links_state.clone();
        let page = *page;

        use_effect_with((page, iteration.clone()), move |_| {
            link_result_state.set(ListLinksState::Loading);

            wasm_bindgen_futures::spawn_local(async move {
                match ShrtApi::get_links(Some(page), None).await {
                    Ok(response) => {
                        link_result_state.set(ListLinksState::Success(response));
                    }
                    Err(e) => {
                        link_result_state.set(ListLinksState::Error(e));
                    }
                }
            });
        });
    }

    let page_num = match &*list_links_state {
        ListLinksState::Success(response) => Some(NonZeroU64::new(response.num_pages).unwrap()),
        _ => None,
    };

    let on_remove_click = |slug: IString| {
        let removing_link_slug = removing_link_slug.clone();

        Callback::from(move |_: MouseEvent| {
            removing_link_slug.set(slug.clone());
        })
    };

    let on_remove: Callback<()> = {
        let iteration = iteration.clone();

        Callback::from(move |_| {
            iteration.set(*iteration + 1);
        })
    };

    let rows = match (*list_links_state).clone() {
        ListLinksState::Success(response) => {
            response.links.into_iter().map(|link| {
                html! {
                    <tr>
                        <td class="text-truncate" style="max-width: 8rem;"><a href={ format!("https://shrt.rs/{}", urlencoding::encode(&link.slug)) }>{ link.slug.clone() }</a></td>
                        <td class="text-truncate" style="max-width: 20rem;"><a href={ link.url.clone() }>{ link.url }</a></td>
                        <td>{ link.visits }</td>
                        <td>{ format_date(link.created_at) }</td>
                        <td class="pt-1 pb-1">
                            <button onclick={ on_remove_click(link.slug.into()) } class="btn btn-danger btn-sm"><i class="bi bi-trash-fill"></i>{ " Remove" }</button>
                        </td>
                    </tr>
                }
            }).collect::<Html>()
        }
        ListLinksState::Loading => {
            (1..=10).map(|_| {
                html!{
                    <tr class="placeholder-glow">
                        <td><span class={ classes!("placeholder", gen_random_col_class()) }></span></td>
                        <td><span class={ classes!("placeholder", gen_random_col_class()) }></span></td>
                        <td><span class={ classes!("placeholder", gen_random_col_class()) }></span></td>
                        <td><span class="placeholder col-8"></span></td>
                        <td><span class="placeholder col-8"></span></td>
                    </tr>
                }
            }).collect::<Html>()
        }
        ListLinksState::Error(_) => {html! {}}
    };

    let on_page_change = {
        let navigator = navigator.clone();

        Callback::from(move |page: NonZeroU64| {
            navigator.push(&Route::ListLinks { page });
        })
    };

    html! {
        <>
            if let Some(error) = (*list_links_state).get_error() {
                <ErrorAlert message={ "Could not retrieve the list of links" } error={Some(Rc::new(error.clone()))} />
            } else {
                <div class="table-responsive">
                    <table class="table table-striped table-hover">
                        <thead>
                            <tr>
                                <th scope="col">{ "Slug" }</th>
                                <th scope="col">{ "URL" }</th>
                                <th scope="col">{ "Visits" }</th>
                                <th scope="col">{ "Created at" }</th>
                                <th scope="col">{ "Actions" }</th>
                            </tr>
                        </thead>
                        <tbody class="table-group-divider">
                            { rows }
                        </tbody>
                    </table>
                </div>

                <Pagination current_page={ *page } page_num={ page_num } on_set_value={ on_page_change } />

                <RemoveLinkModal slug={ (*removing_link_slug).clone() } on_remove={ on_remove } />
            }
        </>
    }
}

fn gen_random_col_class() -> String {
    let cols = ["3", "4", "5", "6", "7", "8", "10", "12"];
    let col = cols.choose(&mut rand::thread_rng()).unwrap();
    format!("col-{}", col)
}

fn format_date(datetime: chrono::DateTime<chrono::Utc>) -> String {
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}
