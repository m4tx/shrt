use std::num::NonZeroU64;

use yew::prelude::*;
use yew_router::components::Link;
use yew_router::{BrowserRouter, Routable, Switch};

use crate::link_result::LinkResult;
use crate::list_links::ListLinks;
use crate::not_found::NotFound;
use crate::url_shortener::UrlShortener;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/app/link/:slug")]
    Link { slug: String },
    #[at("/app/links/:page")]
    ListLinks { page: NonZeroU64 },
    #[at("/404")]
    #[not_found]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <UrlShortener /> },
        Route::Link { slug } => html! { <LinkResult slug={slug} /> },
        Route::ListLinks { page } => html! { <ListLinks page={page} /> },
        Route::NotFound => html! { <NotFound /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <nav class="navbar navbar-expand-md navbar-dark bg-dark mb-4">
                <div class="container">
                    <Link<Route> to={ Route::Home } classes={ classes!("navbar-brand") }>{ "shrt" }</Link<Route>>
                    <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarCollapse" aria-controls="navbarCollapse" aria-expanded="false" aria-label="Toggle navigation">
                        <span class="navbar-toggler-icon"></span>
                    </button>
                    <div class="collapse navbar-collapse" id="navbarCollapse">
                        <ul class="navbar-nav ms-auto mb-2 mb-md-0">
                            <li class="nav-item">
                                <a class="nav-link" href="https://github.com/m4tx/shrt">{ "Source" }</a>
                            </li>
                        </ul>
                    </div>
                </div>
            </nav>

            <main class="container">
                <div class="bg-body-tertiary p-5 rounded">
                    <h1>{ "shrt" }</h1>

                    <Switch<Route> render={switch} />
                </div>
            </main>
        </BrowserRouter>
    }
}
