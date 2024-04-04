use std::num::NonZeroU64;

use gloo_net::http::{Request, Response};
use serde::de::DeserializeOwned;
use shrt_common::errors::ServiceError;
use shrt_common::links::{Link, LinkCreateRequest, LinkExists, LinksResponse};

const API_URL: Option<&str> = option_env!("SHRT_API_URL");

fn api_url() -> &'static str {
    API_URL.unwrap_or("/api")
}

pub struct ShrtApi;

impl ShrtApi {
    pub async fn get_link(slug: &str) -> Result<Link, ServiceError> {
        let result = Request::get(&format!(
            "{}/links/{}",
            api_url(),
            urlencoding::encode(slug)
        ))
        .send()
        .await?;

        Self::map_response(result).await
    }

    pub async fn get_link_exists(slug: &str) -> Result<LinkExists, ServiceError> {
        let result = Request::get(&format!(
            "{}/links/{}/exists",
            api_url(),
            urlencoding::encode(slug)
        ))
        .send()
        .await?;

        Self::map_response(result).await
    }

    pub async fn get_links(
        page: Option<NonZeroU64>,
        links_per_page: Option<NonZeroU64>,
    ) -> Result<LinksResponse, ServiceError> {
        let page = page.unwrap_or(NonZeroU64::new(1).unwrap()).get();
        let links_per_page = links_per_page.unwrap_or(NonZeroU64::new(10).unwrap()).get();

        let result = Request::get(&format!(
            "{}/links?page={}&links_per_page={}",
            api_url(),
            page,
            links_per_page
        ))
        .send()
        .await?;

        Self::map_response(result).await
    }

    pub async fn shorten_url(url: &str, slug: &str) -> Result<Link, ServiceError> {
        let request = LinkCreateRequest {
            url: url.to_string(),
            slug: if slug.is_empty() {
                None
            } else {
                Some(slug.to_string())
            },
        };

        let result = Request::post(&format!("{}/links", api_url()))
            .json(&request)?
            .send()
            .await?;

        Self::map_response(result).await
    }

    async fn map_response<T: DeserializeOwned>(result: Response) -> Result<T, ServiceError> {
        if result.ok() {
            Ok(result.json().await?)
        } else {
            Err(result.json().await?)
        }
    }
}
