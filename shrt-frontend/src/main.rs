mod api;
mod app;
mod error_alert;
mod hand_example;
mod input;
mod link_result;
mod list_links;
mod pagination;
mod remove_link_modal;
mod select;
mod url_shortener;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
