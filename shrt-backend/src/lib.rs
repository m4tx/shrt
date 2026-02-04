use cot::cli::CliMetadata;
use cot::config::ProjectConfig;
use cot::db::migrations::SyncDynMigration;
use cot::project::{MiddlewareContext, RegisterAppsContext, RootHandler, RootHandlerBuilder};
use cot::router::{Route, Router};
use cot::static_files::StaticFilesMiddleware;
use cot::{App, AppBuilder, Project};

pub mod api;
pub mod migrations;
pub mod models;

pub struct LinkApp;

impl App for LinkApp {
    fn name(&self) -> &'static str {
        "links"
    }

    fn migrations(&self) -> Vec<Box<SyncDynMigration>> {
        cot::db::migrations::wrap_migrations(migrations::MIGRATIONS)
    }

    fn router(&self) -> Router {
        use api::*;
        use cot::router::method::get;

        Router::with_urls([
            Route::with_handler("/links/{slug}/exists", get(link_exists)),
            Route::with_handler("/links/{slug}/go", get(redirect_to_link)),
            Route::with_handler("/links/{slug}", get(get_link).delete(remove_link)),
            Route::with_handler("/links", get(get_links).post(create_link)),
        ])
    }
}

pub struct ShrtProject;

impl Project for ShrtProject {
    fn cli_metadata(&self) -> CliMetadata {
        cot::cli::metadata!()
    }

    fn config(&self, config_name: &str) -> cot::Result<ProjectConfig> {
        if config_name == "test" {
            Ok(ProjectConfig::builder()
                .database(
                    cot::config::DatabaseConfig::builder()
                        .url("sqlite::memory:")
                        .build(),
                )
                .build())
        } else {
            Ok(ProjectConfig::dev_default())
        }
    }

    fn middlewares(&self, handler: RootHandlerBuilder, context: &MiddlewareContext) -> RootHandler {
        handler
            .middleware(StaticFilesMiddleware::from_context(context))
            .build()
    }

    fn register_apps(&self, apps: &mut AppBuilder, _context: &RegisterAppsContext) {
        apps.register_with_views(LinkApp, "");
    }
}
