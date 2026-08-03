use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(schemars::JsonSchema))]
pub struct AppConfig {
    pub app_name: String,
    pub base_url: String,
}

#[cfg(feature = "backend")]
impl AppConfig {
    pub fn from_project_config(config: &cot::config::ProjectConfig) -> cot::Result<Self> {
        config
            .extra
            .get("shrt")
            .ok_or_else(|| cot::Error::internal("missing [shrt] section in config file"))?
            .clone()
            .try_into()
            .map_err(cot::Error::internal)
    }
}

#[cfg(feature = "backend")]
impl cot::request::extractors::FromRequestHead for AppConfig {
    async fn from_request_head(head: &cot::request::RequestHead) -> cot::Result<Self> {
        use cot::request::RequestExt as _;
        AppConfig::from_project_config(head.project_config())
    }
}

#[cfg(feature = "backend")]
impl cot::openapi::ApiOperationPart for AppConfig {}
