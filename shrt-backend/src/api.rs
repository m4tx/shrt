use chrono::Utc;
use cot::db::query::{ExprEq, Query};
use cot::db::{Database, Model};
use cot::json::Json;
use cot::request::extractors::{Path, UrlQuery};
use cot::response::{IntoResponse, Response, ResponseExt};
use cot::{Body, StatusCode};
use nanoid::nanoid;
use shrt_common::links::{LinkCreateRequest, LinkExists, LinksResponse};

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
fn to_api_link(link: &Link) -> shrt_common::links::Link {
    shrt_common::links::Link {
        slug: link.slug.clone(),
        url: link.url.clone(),
        created_at: link.created_at,
        visits: link.visits,
    }
}

fn error_response(status: StatusCode, error: &str, message: &str) -> cot::Result<Response> {
    let json = serde_json::json!({
        "error": error,
        "message": message
    });

    Ok(Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(Body::from(json.to_string()))
        .unwrap())
}

pub async fn get_link(db: Database, Path(slug): Path<String>) -> cot::Result<Response> {
    let mut query = Query::<Link>::new();
    query.filter(<Link as Model>::Fields::slug.eq(slug.clone()));
    let link = query.get(&db).await?;

    match link {
        Some(link) => Ok(Json(to_api_link(&link)).into_response()?),
        None => error_response(
            StatusCode::NOT_FOUND,
            "Link not found",
            &format!("Link with slug {} not found", slug),
        ),
    }
}

pub async fn remove_link(db: Database, Path(slug): Path<String>) -> cot::Result<Response> {
    let mut query = Query::<Link>::new();
    query.filter(<Link as Model>::Fields::slug.eq(slug.clone()));
    let result = query.delete(&db).await?;

    if result.rows_affected().0 == 1 {
        Ok(Response::builder()
            .status(StatusCode::NO_CONTENT)
            .body(Body::empty())
            .unwrap())
    } else {
        error_response(
            StatusCode::NOT_FOUND,
            "Link not found",
            &format!("Link with slug {} not found", slug),
        )
    }
}

pub async fn link_exists(db: Database, Path(slug): Path<String>) -> cot::Result<Response> {
    let mut query = Query::<Link>::new();
    query.filter(<Link as Model>::Fields::slug.eq(slug));
    let exists = query.exists(&db).await?;

    Ok(Json(LinkExists { exists }).into_response()?)
}

#[derive(serde::Deserialize)]
pub struct PaginationParams {
    page: Option<u64>,
    links_per_page: Option<u64>,
}

pub async fn get_links(
    db: Database,
    UrlQuery(params): UrlQuery<PaginationParams>,
) -> cot::Result<Response> {
    let page = params.page.unwrap_or(1).max(1);
    let links_per_page = params
        .links_per_page
        .unwrap_or(DEFAULT_LINKS_PER_PAGE)
        .max(1);
    let offset = (page - 1) * links_per_page;

    let total_query = Query::<Link>::new();
    let total_count = total_query.count(&db).await?;
    let num_pages = (total_count + links_per_page - 1) / links_per_page;

    let mut query = Query::<Link>::new();
    query.limit(links_per_page).offset(offset);

    let links = query.all(&db).await?;

    Ok(Json(LinksResponse {
        page,
        links_per_page,
        num_pages,
        links: links.iter().map(to_api_link).collect(),
    })
    .into_response()?)
}

pub async fn create_link(
    db: Database,
    Json(payload): Json<LinkCreateRequest>,
) -> cot::Result<Response> {
    let slug = payload
        .slug
        .unwrap_or_else(|| nanoid!(DEFAULT_SLUG_LENGTH, &ALPHABET));

    // Check if slug exists
    let mut exists_query = Query::<Link>::new();
    exists_query.filter(<Link as Model>::Fields::slug.eq(slug.clone()));
    if exists_query.exists(&db).await? {
        return error_response(
            StatusCode::BAD_REQUEST,
            "Slug already exists",
            &format!("Slug {} already exists", slug),
        );
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
        Ok(_) => Ok(Json(to_api_link(&link)).into_response()?),
        Err(cot::db::DatabaseError::UniqueViolation) => error_response(
            StatusCode::BAD_REQUEST,
            "Slug already exists",
            &format!("Slug {} already exists", slug),
        ),
        Err(e) => Err(e.into()),
    }
}

pub async fn redirect_to_link(db: Database, Path(slug): Path<String>) -> cot::Result<Response> {
    let mut query = Query::<Link>::new();
    query.filter(<Link as Model>::Fields::slug.eq(slug.clone()));
    let link = query.get(&db).await?;

    if let Some(mut link) = link {
        link.visits += 1;
        link.save(&db).await?;

        Ok(Response::builder()
            .status(StatusCode::FOUND)
            .header("Location", link.url)
            .body(Body::empty())
            .unwrap())
    } else {
        error_response(
            StatusCode::NOT_FOUND,
            "Link not found",
            &format!("Link with slug {} not found", slug),
        )
    }
}
