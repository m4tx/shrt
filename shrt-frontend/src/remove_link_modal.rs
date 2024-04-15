use std::rc::Rc;

use serde::{Deserialize, Serialize};
use shrt_common::errors::ServiceError;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::HtmlElement;
use yew::prelude::*;

use crate::api::ShrtApi;
use crate::error_alert::ErrorAlert;

#[wasm_bindgen]
extern "C" {
    type Modal;

    #[wasm_bindgen(constructor, js_namespace = bootstrap)]
    fn new(selector: &str, options: JsValue) -> Modal;

    #[wasm_bindgen(method, js_namespace = bootstrap)]
    fn show(this: &Modal);

    #[wasm_bindgen(method, js_namespace = bootstrap)]
    fn hide(this: &Modal);
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub slug: AttrValue,
    pub on_remove: Callback<()>,
}

#[derive(Clone, Debug, Default)]
enum RemoveLinkModalState {
    Error(ServiceError),
    Loading,
    #[default]
    Initial,
}

impl RemoveLinkModalState {
    #[must_use]
    pub fn is_loading(&self) -> bool {
        matches!(self, RemoveLinkModalState::Loading)
    }

    #[must_use]
    pub fn get_error(&self) -> Option<&ServiceError> {
        match self {
            RemoveLinkModalState::Error(e) => Some(e),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ModalOptions {
    backdrop: bool,
    focus: bool,
    keyboard: bool,
}

#[function_component]
pub fn RemoveLinkModal(props: &Props) -> Html {
    let Props { slug, on_remove } = props;

    let list_links_state = use_state(RemoveLinkModalState::default);
    let modal = use_state(|| None);
    let div_ref = use_node_ref();

    {
        let div_ref = div_ref.clone();
        let modal = modal.clone();

        use_effect_with(div_ref, move |div_ref| {
            let div = div_ref
                .cast::<HtmlElement>()
                .expect("div_ref not attached to div element");

            let options = ModalOptions {
                backdrop: true,
                focus: true,
                keyboard: true,
            };
            let options_js = serde_wasm_bindgen::to_value(&options).unwrap();
            let modal_element = Modal::new(&format!("#{}", div.id()), options_js);
            modal.set(Some(Rc::new(modal_element)));
        });
    }

    {
        let list_links_state = list_links_state.clone();
        let modal = modal.clone();
        let slug = slug.clone();

        use_effect_with(slug.clone(), move |_| {
            if slug.is_empty() {
                return;
            }

            if let Some(modal) = (*modal).clone() {
                list_links_state.set(RemoveLinkModalState::Initial);
                modal.show();
            }
        });
    }

    let on_remove = {
        let list_links_state = list_links_state.clone();
        let modal = modal.clone();
        let slug = slug.clone();
        let on_remove = on_remove.clone();

        Callback::from(move |_: MouseEvent| {
            let list_links_state = list_links_state.clone();
            list_links_state.set(RemoveLinkModalState::Loading);
            let modal = modal.clone();
            let slug = slug.clone();
            let on_remove = on_remove.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match ShrtApi::remove_link(&slug).await {
                    Ok(_) => {
                        list_links_state.set(RemoveLinkModalState::Initial);
                        modal
                            .as_ref()
                            .expect("Modal object has not been created")
                            .hide();
                        on_remove.emit(());
                    }
                    Err(e) => {
                        list_links_state.set(RemoveLinkModalState::Error(e));
                    }
                }
            });
        })
    };

    let is_loading = list_links_state.is_loading();
    let error = list_links_state.get_error();

    html! {
        <div class="modal" ref={ div_ref } id="removeLinkModal" tabindex="-1">
            <div class="modal-dialog">
                <div class="modal-content">
                    <div class="modal-header">
                        <h5 class="modal-title">{ "Remove link" }</h5>
                        <button type="button" class={ classes!("btn-close", is_loading.then_some("disabled")) } data-bs-dismiss="modal" aria-label="Close"></button>
                    </div>
                    <div class="modal-body">
                        <p>{ format!("Are you sure you want to remove link with slug `{}`?", slug) }</p>
                        { error.map(|e| html! { <ErrorAlert message={ "Could not remove the link" } error={ Some(Rc::new(e.clone())) } /> }) }
                    </div>
                    <div class="modal-footer">
                        <button type="button" class={ classes!("btn", "btn-secondary", is_loading.then_some("disabled")) } data-bs-dismiss="modal">{ "Cancel" }</button>
                        <button onclick={ (!is_loading).then_some(on_remove) } type="button" class={ classes!("btn", "btn-danger", is_loading.then_some("disabled")) }>
                            if is_loading {
                                <div class="spinner-border spinner-border-sm" role="status">
                                    <span class="visually-hidden">{ "Loading..." }</span>
                                </div>
                                { " " }
                            }
                            { "Remove" }
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
