use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(schemars::JsonSchema))]
pub struct LinksResponse {
    pub page: u64,
    pub links_per_page: u64,
    pub num_pages: u64,
    pub links: Vec<Link>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(schemars::JsonSchema))]
pub struct Link {
    pub slug: String,
    pub url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub visits: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(schemars::JsonSchema))]
pub struct LinkExists {
    pub exists: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(schemars::JsonSchema))]
pub struct LinkCreateRequest {
    pub slug: Option<String>,
    pub url: String,
}
