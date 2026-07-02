use std::num::NonZeroU64;

use dioxus::prelude::*;

#[component]
pub fn Pagination(
    #[props(default = NonZeroU64::MIN)] current_page: NonZeroU64,
    #[props(default)] page_num: Option<NonZeroU64>,
    on_set_value: EventHandler<NonZeroU64>,
) -> Element {
    let is_loading = page_num.is_none();
    let page_num = page_num.unwrap_or(current_page.saturating_add(2));
    let pages = build_page_elements(current_page, page_num);

    let previous_disabled = current_page.get() == 1;
    let next_disabled = current_page.get() == page_num.get();

    let page_buttons: Vec<Element> = pages
        .into_iter()
        .map(|page_elem| match page_elem {
            PageElement::Page(page) => {
                let is_placeholder = is_loading && page > current_page;
                let is_active = current_page == page;
                rsx! {
                    li { class: if is_active { "page-item active" } else { "page-item" },
                        button {
                            onclick: move |_| {
                                if !is_loading {
                                    on_set_value.call(page);
                                }
                            },
                            class: if is_placeholder { "page-link placeholder" } else { "page-link" },
                            "{page.get()}"
                        }
                    }
                }
            }
            PageElement::Ellipsis => rsx! {
                li { class: "page-item disabled",
                    button { class: "page-link", "…" }
                }
            },
        })
        .collect();

    rsx! {
        nav { "aria-label": "Page navigation",
            ul { class: "pagination justify-content-center placeholder-glow",
                li { class: "page-item",
                    button {
                        onclick: move |_| {
                            on_set_value.call(
                                NonZeroU64::new(current_page.get() - 1)
                                    .unwrap_or(NonZeroU64::MIN),
                            );
                        },
                        class: if previous_disabled { "page-link disabled" } else { "page-link" },
                        "aria-label": "Previous",
                        span { "aria-hidden": "true", "«" }
                    }
                }
                {page_buttons.into_iter()}
                li { class: "page-item",
                    button {
                        onclick: move |_| {
                            if !is_loading {
                                on_set_value.call(current_page.saturating_add(1));
                            }
                        },
                        class: if next_disabled || is_loading {
                            if is_loading { "page-link placeholder" } else { "page-link disabled" }
                        } else {
                            "page-link"
                        },
                        "aria-label": "Next",
                        span { "aria-hidden": "true", "»" }
                    }
                }
            }
        }
    }
}

enum PageElement {
    Page(NonZeroU64),
    Ellipsis,
}

fn build_page_elements(current_page: NonZeroU64, num_pages: NonZeroU64) -> Vec<PageElement> {
    const PAGES_AROUND: u64 = 2;

    let mut pages = Vec::<PageElement>::new();

    let current_page = current_page.get();
    let num_pages = num_pages.get();

    if 1 < current_page.saturating_sub(PAGES_AROUND) {
        pages.push(PageElement::Page(NonZeroU64::new(1).unwrap()));
        if 1 < current_page.saturating_add(PAGES_AROUND + 1) {
            pages.push(PageElement::Ellipsis);
        }
    }
    for page in
        current_page.saturating_sub(PAGES_AROUND)..=current_page.saturating_add(PAGES_AROUND)
    {
        if page > 0 && page <= num_pages {
            pages.push(PageElement::Page(NonZeroU64::new(page).unwrap()));
        }
    }
    if num_pages > current_page.saturating_add(PAGES_AROUND) {
        if num_pages > current_page.saturating_add(PAGES_AROUND) + 1 {
            pages.push(PageElement::Ellipsis);
        }
        pages.push(PageElement::Page(NonZeroU64::new(num_pages).unwrap()));
    }

    pages
}
