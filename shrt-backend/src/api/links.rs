use std::num::NonZeroU64;

use rocket::http::Status;
use rocket::response::status::NoContent;
use rocket::serde::json::Json;
use rocket::{delete, get, post};
use rocket_okapi::openapi;
use sea_orm::sea_query::Expr;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DbConn, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder,
};
use sea_orm_rocket::Connection;
use shrt_common::errors::ServiceError;
use shrt_common::links::{Link, LinkCreateRequest, LinkExists, LinksResponse};
use shrt_entity::link;

use crate::pool::Db;

const DEFAULT_LINKS_PER_PAGE: u64 = 30;

type ApiResult<T> = Result<Json<T>, ServiceError>;
type NoContentApiResult = Result<NoContent, ServiceError>;
pub type DataResult<'a, T> = Result<Json<T>, rocket::serde::json::Error<'a>>;

#[derive(Debug)]
struct LinkDb;

impl LinkDb {
    pub async fn create_link(
        db: &DbConn,
        link: link::ActiveModel,
    ) -> Result<link::Model, LinkDbError> {
        Ok(link.insert(db).await?)
    }

    pub async fn exists(db: &DbConn, slug: &str) -> Result<bool, LinkDbError> {
        let count = link::Entity::find()
            .filter(link::Column::Slug.contains(slug))
            .count(db)
            .await?;
        Ok(count > 0)
    }

    pub async fn find_link_by_slug(db: &DbConn, slug: &str) -> Result<link::Model, LinkDbError> {
        link::Entity::find()
            .filter(link::Column::Slug.contains(slug))
            .one(db)
            .await?
            .ok_or(LinkDbError::not_found(slug.to_owned()))
    }

    pub async fn delete_link_by_slug(db: &DbConn, slug: &str) -> Result<(), LinkDbError> {
        let result = link::Entity::delete_many()
            .filter(link::Column::Slug.eq(slug))
            .exec(db)
            .await?;

        if result.rows_affected != 1 {
            Err(LinkDbError::not_found(slug.to_owned()))
        } else {
            Ok(())
        }
    }

    pub async fn increment_visits(db: &DbConn, slug: &str) -> Result<(), LinkDbError> {
        let update_result = link::Entity::update_many()
            .col_expr(link::Column::Visits, Expr::col(link::Column::Visits).add(1))
            .filter(link::Column::Slug.eq(slug))
            .exec(db)
            .await?;

        if update_result.rows_affected != 1 {
            return Err(LinkDbError::not_found(slug.to_owned()));
        }
        Ok(())
    }

    pub async fn find_links_in_page(
        db: &DbConn,
        page: u64,
        links_per_page: u64,
    ) -> Result<(Vec<link::Model>, u64), LinkDbError> {
        let paginator = link::Entity::find()
            .order_by_desc(link::Column::CreatedAt)
            .paginate(db, links_per_page);
        let num_pages = paginator.num_pages().await?;

        Ok(paginator
            .fetch_page(page - 1)
            .await
            .map(|links| (links, num_pages))?)
    }
}

#[derive(Debug)]
enum LinkDbError {
    DbError(DbErr),
    NotFound { slug: String },
}

impl LinkDbError {
    #[must_use]
    fn not_found(slug: String) -> Self {
        Self::NotFound { slug }
    }
}

impl From<DbErr> for LinkDbError {
    fn from(value: DbErr) -> Self {
        LinkDbError::DbError(value)
    }
}

impl From<LinkDbError> for ServiceError {
    fn from(value: LinkDbError) -> Self {
        match value {
            LinkDbError::NotFound { slug } => ServiceError {
                error: "Link not found".to_string(),
                message: Some(format!("Link with slug {} not found", slug)),
                http_status: Status::NotFound,
            },
            LinkDbError::DbError(db_error) => ServiceError {
                error: "Database error".to_string(),
                message: Some(db_error.to_string()),
                http_status: Status::InternalServerError,
            },
        }
    }
}

#[openapi]
#[get("/links/<slug>")]
pub async fn get_link(conn: Connection<'_, Db>, slug: &str) -> ApiResult<Link> {
    let db = conn.into_inner();

    let link = LinkDb::find_link_by_slug(db, slug).await?;

    Ok(Json(link.into()))
}

#[openapi]
#[delete("/links/<slug>")]
pub async fn remove_link(conn: Connection<'_, Db>, slug: &str) -> NoContentApiResult {
    let db = conn.into_inner();

    LinkDb::delete_link_by_slug(db, slug).await?;

    Ok(NoContent)
}

#[openapi]
#[get("/links/<slug>/exists")]
pub async fn link_exists(conn: Connection<'_, Db>, slug: &str) -> ApiResult<LinkExists> {
    let db = conn.into_inner();

    let link_exists = LinkDb::exists(db, slug).await?;

    Ok(Json(LinkExists {
        exists: link_exists,
    }))
}

#[openapi]
#[get("/links?<page>&<links_per_page>")]
pub async fn get_links(
    conn: Connection<'_, Db>,
    page: Option<NonZeroU64>,
    links_per_page: Option<NonZeroU64>,
) -> ApiResult<LinksResponse> {
    let db = conn.into_inner();

    let page = page.unwrap_or(NonZeroU64::new(1).unwrap()).get();
    let links_per_page = links_per_page
        .unwrap_or(NonZeroU64::new(DEFAULT_LINKS_PER_PAGE).unwrap())
        .get();

    let (links, num_pages) = LinkDb::find_links_in_page(db, page, links_per_page).await?;

    Ok(Json(LinksResponse {
        page,
        links_per_page,
        num_pages,
        links: links.into_iter().map(|x| x.into()).collect(),
    }))
}

/// The alphabet to generate the links from. Essentially [0-9A-Za-z], but with
/// '0', 'o', 'O', '1', 'l', 'I' removed to avoid confusion (Base56 alphabet).
const ALPHABET: [char; 56] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k',
    'm', 'n', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F',
    'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

const DEFAULT_SLUG_LENGTH: usize = 7;

#[openapi]
#[post("/links", data = "<post_data>")]
pub async fn create_link(
    conn: Connection<'_, Db>,
    post_data: DataResult<'_, LinkCreateRequest>,
) -> ApiResult<Link> {
    let post_data = post_data.expect("Cannot parse request body").into_inner();
    let slug = post_data
        .slug
        .unwrap_or_else(|| nanoid::nanoid!(DEFAULT_SLUG_LENGTH, &ALPHABET));

    let link_model = LinkDb::create_link(
        conn.into_inner(),
        link::ActiveModel {
            id: ActiveValue::not_set(),
            slug: ActiveValue::set(slug),
            url: ActiveValue::set(post_data.url),
            created_at: ActiveValue::set(chrono::Utc::now()),
            visits: ActiveValue::set(0),
        },
    )
    .await?;

    Ok(Json(link_model.into()))
}

#[openapi]
#[get("/links/<slug>/go")]
pub async fn redirect_to_link(
    conn: Connection<'_, Db>,
    slug: &str,
) -> Result<rocket::response::Redirect, ServiceError> {
    // TODO transaction
    let db = conn.into_inner();
    let link = LinkDb::find_link_by_slug(db, slug).await?;

    LinkDb::increment_visits(db, slug).await?;
    Ok(rocket::response::Redirect::to(link.url))
}
