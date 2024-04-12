use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use yew_hooks::prelude::*;

const DEBOUNCE_TIMEOUT: u32 = 500;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub id: AttrValue,
    #[prop_or_default]
    pub value: AttrValue,
    #[prop_or_default]
    pub placeholder: AttrValue,
    #[prop_or_default]
    pub error: AttrValue,
    #[prop_or(AttrValue::from("text"))]
    pub input_type: AttrValue,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub required: bool,
    pub on_set_value: Callback<String>,
    #[prop_or_default]
    pub on_debounce: Callback<String>,
}

#[function_component]
pub fn Input(props: &Props) -> Html {
    let value = use_state(|| props.value.to_string());

    let debounce = {
        let props = props.clone();

        let value = value.clone();
        use_debounce(
            move || {
                props.on_debounce.emit((*value).clone());
            },
            DEBOUNCE_TIMEOUT,
        )
    };

    let on_change = {
        let value_debounced = value;
        let on_set_value = props.on_set_value.clone();

        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                value_debounced.set(input.value());
                on_set_value.emit(input.value());
                debounce.run();
            }
        })
    };

    let has_error = !(*props.error).is_empty();
    html! {
        <>
            <input
                oninput={ on_change }
                id={ props.id.clone() }
                value={ (*props.value).to_string() }
                placeholder={ (*props.placeholder).to_string() }
                class={ classes!("form-control", has_error.then_some(Some("is-invalid"))) }
                type={ (*props.input_type).to_string() }
                disabled={ props.disabled }
                required={ props.required }
            />
            if has_error {
                <div class="invalid-feedback">
                    { (*props.error).to_string() }
                </div>
            }
        </>
    }
}
