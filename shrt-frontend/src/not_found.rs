use yew::prelude::*;
use yew_router::components::Link;

use crate::app::Route;

#[function_component]
pub fn NotFound() -> Html {
    html! {
        <>
            <div class="mb-3">
                <h1 class="display-2">{ "Page not found" }</h1>
                <p class="lead">{ "The page you have requested could not be found." }</p>

                <Link<Route> to={ Route::Home } classes={ classes!("btn", "btn-primary") }><i class="bi bi-house-fill"></i>{ " Go to homepage" }</Link<Route>>
            </div>
        </>
    }
}
