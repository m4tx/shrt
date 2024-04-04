use std::rc::Rc;

use shrt_common::errors::ServiceError;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub message: AttrValue,
    #[prop_or_default]
    pub error: Option<Rc<ServiceError>>,
}

#[function_component]
pub fn ErrorAlert(props: &Props) -> Html {
    let Props { message, error } = props;

    let displayed_message = {
        let mut s = String::new();

        if !message.is_empty() {
            s.push_str(message);
        }
        if let Some(error) = error {
            if !s.is_empty() {
                s.push_str(": ");
            }

            s.push_str(&error.error);
            if let Some(error_message) = &error.message {
                s.push_str(": ");
                s.push_str(error_message);
            }
        }

        s
    };

    html! {
        <div class="alert alert-danger d-flex align-items-center" role="alert">
            <i class="bi bi-exclamation-triangle-fill flex-shrink-0 me-2"></i>
            <div>
                { displayed_message }
            </div>
        </div>
    }
}
