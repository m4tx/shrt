#[cfg(feature = "backend")]
use cot::response::ResponseExt;
use serde::{Deserialize, Serialize};

/// Error messages returned to user
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(schemars::JsonSchema))]
pub struct ServiceError {
    /// The title of the error message
    pub error: String,
    /// The description of the error
    pub message: Option<String>,
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "Error `{}`: {}",
            self.error,
            self.message.as_deref().unwrap_or("<no message>")
        )
    }
}

impl std::error::Error for ServiceError {}

#[cfg(feature = "backend")]
impl cot::response::IntoResponse for ServiceError {
    fn into_response(self) -> cot::Result<cot::response::Response> {
        let body = serde_json::to_string(&self).unwrap();
        Ok(cot::response::Response::builder()
            .status(cot::StatusCode::INTERNAL_SERVER_ERROR)
            .header("Content-Type", "application/json")
            .body(cot::Body::from(body))
            .unwrap())
    }
}

#[cfg(feature = "frontend")]
impl From<gloo_net::Error> for ServiceError {
    fn from(err: gloo_net::Error) -> Self {
        Self {
            error: "Request Error".to_owned(),
            message: Some(err.to_string()),
        }
    }
}
