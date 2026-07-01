use serde::{Deserialize, Serialize};

/// Error messages returned to user
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(cot::schemars::JsonSchema))]
pub struct ServiceError {
    /// The title of the error message
    pub error: String,
    /// The description of the error
    pub message: Option<String>,
    /// HTTP status code
    #[serde(skip)]
    #[cfg(feature = "backend")]
    pub status: cot::StatusCode,
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
        use cot::response::ResponseExt;
        let body = serde_json::to_string(&self).unwrap();
        Ok(cot::response::Response::builder()
            .status(self.status)
            .header("Content-Type", "application/json")
            .body(cot::Body::from(body))
            .unwrap())
    }
}

#[cfg(feature = "backend")]
impl From<ServiceError> for cot::Error {
    fn from(value: ServiceError) -> Self {
        cot::Error::wrap(value)
    }
}

#[cfg(feature = "backend")]
impl cot::openapi::ApiOperationResponse for ServiceError {
    fn api_operation_responses(
        _operation: &mut cot::aide::openapi::Operation,
        _route_context: &cot::openapi::RouteContext<'_>,
        schema_generator: &mut cot::schemars::SchemaGenerator,
    ) -> Vec<(
        Option<cot::aide::openapi::StatusCode>,
        cot::aide::openapi::Response,
    )> {
        use cot::aide::openapi::{MediaType, Response};
        use cot::schemars::JsonSchema;
        use indexmap::IndexMap;

        vec![(
            None, // Default response for errors
            Response {
                description: "Error response".to_string(),
                content: IndexMap::from([(
                    "application/json".to_string(),
                    MediaType {
                        schema: Some(cot::aide::openapi::SchemaObject {
                            json_schema: Self::json_schema(schema_generator),
                            external_docs: None,
                            example: None,
                        }),
                        ..Default::default()
                    },
                )]),
                ..Default::default()
            },
        )]
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
