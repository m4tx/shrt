use log::LevelFilter;
use migration::MigratorTrait;
use rocket::fairing::AdHoc;
use rocket::{fairing, Build, Rocket};
use rocket_okapi::openapi_get_routes;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use sea_orm_rocket::Database;

use crate::logging::init_logging;
use crate::pool::Db;

mod api;
mod logging;
mod pool;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    init_logging(LevelFilter::Debug).expect("Could not initialize logging");

    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount(
            "/",
            openapi_get_routes![
                api::links::get_link,
                api::links::link_exists,
                api::links::get_links,
                api::links::create_link,
                api::links::redirect_to_link
            ],
        )
        .mount("/docs", make_swagger_ui(&get_docs()))
        .launch()
        .await?;

    Ok(())
}

pub fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "/openapi.json".to_string(),
        deep_linking: true,
        ..Default::default()
    }
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}
