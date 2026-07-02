use dioxus::prelude::*;
use shrt_common::errors::ServiceError;

fn run_js(script: &str) {
    js_sys::eval(script).ok();
}

use crate::api::ShrtApi;
use crate::error_alert::ErrorAlert;

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

#[component]
pub fn RemoveLinkModal(#[props(default)] slug: String, on_remove: EventHandler<()>) -> Element {
    let mut state: Signal<RemoveLinkModalState> = use_signal(RemoveLinkModalState::default);
    let mut slug_signal = use_signal(String::new);

    if *slug_signal.peek() != slug {
        slug_signal.set(slug.clone());
    }

    use_effect(move || {
        let s = slug_signal();
        if s.is_empty() {
            return;
        }
        state.set(RemoveLinkModalState::Initial);
        run_js(
            "bootstrap.Modal.getOrCreateInstance(\
             document.getElementById('removeLinkModal'),\
             {backdrop:true,focus:true,keyboard:true}).show()",
        );
    });

    let is_loading = state.read().is_loading();
    let error = state.read().get_error().cloned();

    rsx! {
        div { class: "modal", id: "removeLinkModal", tabindex: "-1",
            div { class: "modal-dialog",
                div { class: "modal-content",
                    div { class: "modal-header",
                        h5 { class: "modal-title", "Remove link" }
                        button {
                            r#type: "button",
                            class: if is_loading { "btn-close disabled" } else { "btn-close" },
                            "data-bs-dismiss": "modal",
                            "aria-label": "Close",
                        }
                    }
                    div { class: "modal-body",
                        p { "Are you sure you want to remove link with slug `{slug}`?" }
                        if let Some(e) = error {
                            ErrorAlert { message: "Could not remove the link", error: Some(e) }
                        }
                    }
                    div { class: "modal-footer",
                        button {
                            r#type: "button",
                            class: if is_loading { "btn btn-secondary disabled" } else { "btn btn-secondary" },
                            "data-bs-dismiss": "modal",
                            "Cancel"
                        }
                        button {
                            r#type: "button",
                            class: if is_loading { "btn btn-danger disabled" } else { "btn btn-danger" },
                            onclick: move |_| {
                                if !is_loading {
                                    let s = slug.clone();
                                    state.set(RemoveLinkModalState::Loading);
                                    spawn(async move {
                                        match ShrtApi::remove_link(&s).await {
                                            Ok(_) => {
                                                state.set(RemoveLinkModalState::Initial);
                                                run_js(
                                                    "bootstrap.Modal.getInstance(\
                                                     document.getElementById('removeLinkModal')).hide()",
                                                );
                                                on_remove.call(());
                                            }
                                            Err(e) => {
                                                state.set(RemoveLinkModalState::Error(e));
                                            }
                                        }
                                    });
                                }
                            },
                            if is_loading {
                                div { class: "spinner-border spinner-border-sm", role: "status",
                                    span { class: "visually-hidden", "Loading..." }
                                }
                                " "
                            }
                            "Remove"
                        }
                    }
                }
            }
        }
    }
}
