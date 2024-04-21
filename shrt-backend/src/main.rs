use clap::{Parser, Subcommand};
use const_format::concatcp;
use log::LevelFilter;
use migration::MigratorTrait;
use rocket::fairing::AdHoc;
use rocket::{fairing, Build, Rocket};
use rocket_okapi::openapi_get_routes;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use sea_orm_rocket::Database;
use shadow_rs::shadow;

use crate::api::auth::create_user;
use crate::logging::init_logging;
use crate::pool::Db;

mod api;
mod logging;
mod pool;

shadow!(build);

const VERSION_STRING: &str = concatcp!(build::PKG_VERSION, " (commit ", build::SHORT_COMMIT, ")");

#[derive(Parser, Debug)]
#[command(author, version = VERSION_STRING, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

impl Cli {
    pub fn command(&self) -> Command {
        self.command.clone().unwrap_or_default()
    }
}

#[derive(Subcommand, Debug, Clone, Default)]
enum Command {
    CreateSuperuser,
    #[default]
    RunServer,
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let cli = Cli::parse();

    init_logging(LevelFilter::Debug).expect("Could not initialize logging");

    match &cli.command() {
        Command::CreateSuperuser => {
            create_rocket()
                .attach(AdHoc::try_on_ignite("CreateSuperuser", create_superuser))
                .ignite()
                .await?;
        }
        Command::RunServer => {
            create_rocket()
                .mount(
                    "/",
                    openapi_get_routes![
                        api::auth::login,
                        api::auth::change_password,
                        api::links::get_link,
                        api::links::link_exists,
                        api::links::get_links,
                        api::links::create_link,
                        api::links::remove_link,
                        api::links::redirect_to_link,
                    ],
                )
                .mount("/docs", make_swagger_ui(&get_docs()))
                .launch()
                .await?;
        }
    }

    Ok(())
}

fn create_rocket() -> Rocket<Build> {
    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
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

async fn create_superuser(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    create_user(conn, "admin", "admin")
        .await
        .expect("Could not create superuser");
    Ok(rocket)
}
