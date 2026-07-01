use cot::cli::CliMetadata;
use cot::config::ProjectConfig;
use cot::db::migrations::SyncDynMigration;
use cot::openapi::swagger_ui::SwaggerUi;
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
        use cot::openapi::NoApi;
        use cot::router::method::openapi::ApiMethodRouter;

        Router::with_urls([
            Route::with_api_handler(
                "/links/{slug}/exists",
                ApiMethodRouter::new().get(link_exists),
            ),
            Route::with_api_handler(
                "/links/{slug}/go",
                ApiMethodRouter::new().get(NoApi(redirect_to_link)),
            ),
            Route::with_api_handler(
                "/links/{slug}",
                ApiMethodRouter::new()
                    .get(get_link)
                    .delete(NoApi(remove_link)),
            ),
            Route::with_api_handler(
                "/links",
                ApiMethodRouter::new().get(get_links).post(create_link),
            ),
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
        apps.register_with_views(SwaggerUi::new(), "/swagger");
        apps.register_with_views(LinkApp, "");
    }
}
