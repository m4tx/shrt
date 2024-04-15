mod api;
mod app;
mod error_alert;
mod hand_example;
mod input;
mod link_result;
mod list_links;
mod not_found;
mod pagination;
mod remove_link_modal;
mod select;
mod url_shortener;

use app::App;
use log::Level;

fn main() {
    console_log::init_with_level(Level::Debug).expect("Could not initialize logger");
    yew::Renderer::<App>::new().render();
}
