use gloo_console::log;
use serde::{Deserialize, Serialize};
use shrt_common::errors::ServiceError;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    type Modal;
    #[wasm_bindgen(constructor, js_namespace = bootstrap)]
    fn new(selector: &str, options: JsValue) -> Modal;
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub slug: AttrValue,
}

#[derive(Clone, Debug, Default)]
enum RemoveLinkModalState {
    Error(ServiceError),
    Loading,
    #[default]
    Initial,
}

#[derive(Serialize, Deserialize)]
struct ModalOptions {
    backdrop: bool,
    focus: bool,
    keyboard: bool,
}

#[function_component]
pub fn RemoveLinkModal(props: &Props) -> Html {
    let Props { slug } = props;

    // const myModalAlternative = new bootstrap.Modal('#myModal', options)
    // Document::new()
    //     .expect("document should exist")
    //     .
    let options = ModalOptions {
        backdrop: true,
        focus: true,
        keyboard: true,
    };
    let options_js = serde_wasm_bindgen::to_value(&options).unwrap();
    log!("options_js: ", &options_js);
    let modal = Modal::new("#myModal", options_js);

    html! {
        <div class="modal" id="myModal" tabindex="-1">
            <div class="modal-dialog">
                <div class="modal-content">
                    <div class="modal-header">
                        <h5 class="modal-title">{ "Modal title" }</h5>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                    </div>
                    <div class="modal-body">
                        <p>{ "Modal body text goes here." }</p>
                    </div>
                    <div class="modal-footer">
                        <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{ "Close" }</button>
                        <button type="button" class="btn btn-primary">{ "Save changes" }</button>
                    </div>
                </div>
            </div>
        </div>
    }
}
