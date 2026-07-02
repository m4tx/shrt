use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;

const DEBOUNCE_TIMEOUT_MS: u32 = 500;

#[component]
pub fn Input(
    #[props(default)] id: String,
    value: String,
    #[props(default)] placeholder: String,
    #[props(default)] error: String,
    #[props(default = "text".to_string())] input_type: String,
    #[props(default)] disabled: bool,
    #[props(default)] required: bool,
    on_set_value: EventHandler<String>,
    #[props(default)] on_debounce: Option<EventHandler<String>>,
) -> Element {
    let mut debounce_gen: Signal<u64> = use_signal(|| 0u64);
    let mut debounce_val: Signal<String> = use_signal(String::new);

    use_effect(move || {
        let epoch = debounce_gen();
        let val = debounce_val.peek().clone();

        if epoch == 0 {
            return;
        }

        spawn(async move {
            TimeoutFuture::new(DEBOUNCE_TIMEOUT_MS).await;
            if *debounce_gen.peek() == epoch
                && let Some(handler) = on_debounce
            {
                handler.call(val);
            }
        });
    });

    let has_error = !error.is_empty();
    let class = if has_error {
        "form-control is-invalid"
    } else {
        "form-control"
    };

    rsx! {
        input {
            oninput: move |e| {
                let v = e.value();
                debounce_val.set(v.clone());
                *debounce_gen.write() += 1;
                on_set_value.call(v);
            },
            id: id.clone(),
            value: value.clone(),
            placeholder: placeholder.clone(),
            class: class,
            r#type: input_type.clone(),
            disabled: disabled,
            required: required,
        }
        if has_error {
            div { class: "invalid-feedback", {error} }
        }
    }
}
