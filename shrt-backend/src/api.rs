use chrono::Utc;
use cot::db::{query, Database, Model, StatementResult};
use cot::json::Json;
use cot::request::extractors::{Path, UrlQuery};
use cot::response::Redirect;
use cot::StatusCode;
use nanoid::nanoid;
use shrt_common::errors::ServiceError;
use shrt_common::links::{Link as ApiLink, LinkCreateRequest, LinkExists, LinksResponse};

use crate::models::Link;

const DEFAULT_LINKS_PER_PAGE: u64 = 30;
const DEFAULT_SLUG_LENGTH: usize = 7;
/// The alphabet to generate the links from. Essentially [0-9A-Za-z], but with
/// '0', 'o', 'O', '1', 'l', 'I' removed to avoid confusion (Base56 alphabet).
const ALPHABET: [char; 56] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k',
    'm', 'n', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F',
    'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

// Helper to convert DB Link to API Link
fn to_api_link(link: &Link) -> ApiLink {
    ApiLink {
        slug: link.slug.clone(),
        url: link.url.clone(),
        created_at: link.created_at,
        visits: link.visits,
    }
}

fn error(status: StatusCode, error: &str, message: &str) -> ServiceError {
    ServiceError {
        status,
        error: error.to_string(),
        message: Some(message.to_string()),
    }
}

pub async fn get_link(
    db: Database,
    Path(slug): Path<String>,
) -> Result<Json<ApiLink>, ServiceError> {
    let link: Option<Link> = query!(Link, $slug == slug.clone())
        .get(&db)
        .await
        .map_err(|e| {
            error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error",
                &e.to_string(),
            )
        })?;

    match link {
        Some(link) => Ok(Json(to_api_link(&link))),
        None => Err(error(
            StatusCode::NOT_FOUND,
            "Link not found",
            &format!("Link with slug {} not found", slug),
        )),
    }
}

pub async fn remove_link(
    db: Database,
    Path(slug): Path<String>,
) -> Result<StatusCode, ServiceError> {
    let result: StatementResult = query!(Link, $slug == slug.clone())
        .delete(&db)
        .await
        .map_err(|e| {
            error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error",
                &e.to_string(),
            )
        })?;

    if result.rows_affected().0 == 1 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(error(
            StatusCode::NOT_FOUND,
            "Link not found",
            &format!("Link with slug {} not found", slug),
        ))
    }
}

pub async fn link_exists(
    db: Database,
    Path(slug): Path<String>,
) -> Result<Json<LinkExists>, ServiceError> {
    let exists: bool = query!(Link, $slug == slug).exists(&db).await.map_err(|e| {
        error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error",
            &e.to_string(),
        )
    })?;

    Ok(Json(LinkExists { exists }))
}

#[derive(serde::Deserialize, cot::schemars::JsonSchema)]
pub struct PaginationParams {
    page: Option<u64>,
    links_per_page: Option<u64>,
}

pub async fn get_links(
    db: Database,
    UrlQuery(params): UrlQuery<PaginationParams>,
) -> Result<Json<LinksResponse>, ServiceError> {
    let page = params.page.unwrap_or(1).max(1);
    let links_per_page = params
        .links_per_page
        .unwrap_or(DEFAULT_LINKS_PER_PAGE)
        .max(1);
    let offset = (page - 1) * links_per_page;

    let total_count = Link::objects().count(&db).await.map_err(|e| {
        error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error",
            &e.to_string(),
        )
    })?;
    let num_pages = total_count.div_ceil(links_per_page);

    let links = Link::objects()
        .limit(links_per_page)
        .offset(offset)
        .all(&db)
        .await
        .map_err(|e| {
            error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error",
                &e.to_string(),
            )
        })?;

    Ok(Json(LinksResponse {
        page,
        links_per_page,
        num_pages,
        links: links.iter().map(to_api_link).collect(),
    }))
}

pub async fn create_link(
    db: Database,
    Json(payload): Json<LinkCreateRequest>,
) -> Result<Json<ApiLink>, ServiceError> {
    let slug = payload
        .slug
        .clone()
        .unwrap_or_else(|| nanoid!(DEFAULT_SLUG_LENGTH, &ALPHABET));

    // Check if slug exists
    let exists: bool = query!(Link, $slug == slug.clone())
        .exists(&db)
        .await
        .map_err(|e| {
            error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error",
                &e.to_string(),
            )
        })?;
    if exists {
        return Err(error(
            StatusCode::BAD_REQUEST,
            "Slug already exists",
            &format!("Slug {} already exists", slug),
        ));
    }

    let mut link = Link {
        id: cot::db::Auto::auto(),
        slug: slug.clone(),
        url: payload.url,
        created_at: Utc::now(),
        visits: 0,
    };

    // Use insert to catch potential race condition if check above passed but
    // another request inserted same slug
    match link.insert(&db).await {
        Ok(_) => Ok(Json(to_api_link(&link))),
        Err(cot::db::DatabaseError::UniqueViolation) => Err(error(
            StatusCode::BAD_REQUEST,
            "Slug already exists",
            &format!("Slug {} already exists", slug),
        )),
        Err(e) => Err(error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error",
            &e.to_string(),
        )),
    }
}

pub async fn redirect_to_link(
    db: Database,
    Path(slug): Path<String>,
) -> Result<Redirect, ServiceError> {
    let link: Option<Link> = query!(Link, $slug == slug.clone())
        .get(&db)
        .await
        .map_err(|e| {
            error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error",
                &e.to_string(),
            )
        })?;

    if let Some(mut link) = link {
        link.visits += 1;
        link.update(&db).await.map_err(|e| {
            error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error",
                &e.to_string(),
            )
        })?;

        Ok(Redirect::new(link.url))
    } else {
        Err(error(
            StatusCode::NOT_FOUND,
            "Link not found",
            &format!("Link with slug {} not found", slug),
        ))
    }
}
