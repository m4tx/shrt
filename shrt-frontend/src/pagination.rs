use std::num::NonZeroU64;

use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or(NonZeroU64::new(1).unwrap())]
    pub current_page: NonZeroU64,
    #[prop_or_default]
    pub page_num: Option<NonZeroU64>,
    pub on_set_value: Callback<NonZeroU64>,
}

enum PageElement {
    Page(NonZeroU64),
    Ellipsis,
}

#[function_component]
pub fn Pagination(props: &Props) -> Html {
    let Props {
        current_page,
        page_num,
        on_set_value,
    } = props;

    let on_change = |page: NonZeroU64| {
        let on_set_value = on_set_value.clone();

        Callback::from(move |_: MouseEvent| {
            on_set_value.emit(page);
        })
    };

    let is_loading = page_num.is_none();

    let page_num = page_num.unwrap_or(current_page.saturating_add(2));
    let pages = build_page_elements(*current_page, page_num);

    let previous_disabled = current_page.get() == 1;
    let next_disabled = current_page.get() == page_num.get();

    html! {
        <nav aria-label="Page navigation">
            <ul class="pagination justify-content-center placeholder-glow">
                <li class="page-item">
                    <button onclick={ on_change(NonZeroU64::new(current_page.get() - 1).unwrap_or(NonZeroU64::MIN)) } class={ classes!("page-link", previous_disabled.then_some("disabled")) } aria-label="Previous">
                        <span aria-hidden="true">{ "«" }</span>
                    </button>
                </li>
                {
                    for pages.into_iter().map(|page| match page {
                        PageElement::Page(page) => {
                            let is_placeholder = is_loading && page > *current_page;
                            html! {
                                <li class={ classes!("page-item", (*current_page == page).then_some("active")) }>
                                    <button onclick={ (!is_loading).then(|| on_change(page)) } class={ classes!("page-link", is_placeholder.then_some("placeholder")) }>{ page.get() }</button>
                                </li>
                            }
                        },
                        PageElement::Ellipsis => html! {
                            <li class="page-item disabled">
                                <button class="page-link">{ "…" }</button>
                            </li>
                        },
                    })
                }
                <li class="page-item">
                    <button onclick={ (!is_loading).then(|| on_change(current_page.saturating_add(1))) } class={ classes!("page-link", next_disabled.then_some("disabled"), is_loading.then_some("placeholder")) } aria-label="Next">
                        <span aria-hidden="true">{ "»" }</span>
                    </button>
                </li>
            </ul>
        </nav>
    }
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
