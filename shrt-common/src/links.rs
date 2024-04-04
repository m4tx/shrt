#[cfg(feature = "backend")]
use rocket_okapi::okapi::schemars::{self, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(JsonSchema))]
#[cfg_attr(feature = "backend", serde(crate = "rocket::serde"))]
pub struct LinksResponse {
    pub page: u64,
    pub links_per_page: u64,
    pub num_pages: u64,
    pub links: Vec<Link>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(JsonSchema))]
#[cfg_attr(feature = "backend", serde(crate = "rocket::serde"))]
pub struct Link {
    pub slug: String,
    pub url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub visits: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(JsonSchema))]
#[cfg_attr(feature = "backend", serde(crate = "rocket::serde"))]
pub struct LinkExists {
    pub exists: bool,
}

#[cfg(feature = "backend")]
impl From<shrt_entity::link::Model> for Link {
    fn from(value: shrt_entity::link::Model) -> Self {
        Self {
            slug: value.slug,
            url: value.url,
            created_at: value.created_at,
            visits: value.visits,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(JsonSchema))]
#[cfg_attr(feature = "backend", serde(crate = "rocket::serde"))]
pub struct LinkCreateRequest {
    pub slug: Option<String>,
    pub url: String,
}
