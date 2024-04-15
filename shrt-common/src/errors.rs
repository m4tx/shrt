#[cfg(feature = "backend")]
use rocket::{
    http::{ContentType, Status},
    request::Request,
    response::{self, Responder, Response},
};
#[cfg(feature = "backend")]
use rocket_okapi::okapi::openapi3::Responses;
#[cfg(feature = "backend")]
use rocket_okapi::okapi::schemars::Map;
#[cfg(feature = "backend")]
use rocket_okapi::{
    gen::OpenApiGenerator, response::OpenApiResponderInner, JsonSchema, OpenApiError,
};
use serde::{Deserialize, Serialize};

/// Error messages returned to user
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(JsonSchema))]
pub struct ServiceError {
    /// The title of the error message
    pub error: String,
    /// The description of the error
    pub message: Option<String>,
    /// HTTP Status returned
    #[serde(skip)]
    #[cfg(feature = "backend")]
    pub http_status: Status,
}

#[cfg(feature = "backend")]
impl OpenApiResponderInner for ServiceError {
    fn responses(_generator: &mut OpenApiGenerator) -> Result<Responses, OpenApiError> {
        use rocket_okapi::okapi::openapi3::{RefOr, Response as OpenApiReponse};

        let mut responses = Map::new();
        responses.insert(
            "200".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "\
                **OK**\n\n\
                The request has succeeded. \
                "
                .to_string(),
                ..Default::default()
            }),
        );
        responses.insert(
            "204".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "\
                **No Content**\n\n\
                The request has succeeded and there is no content to return. \
                "
                .to_string(),
                ..Default::default()
            }),
        );
        responses.insert(
            "400".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "\
                **Bad Request**\n\n\
                The request given is wrongly formatted or data asked could not be fulfilled. \
                "
                .to_string(),
                ..Default::default()
            }),
        );
        responses.insert(
            "404".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "\
                **Not Found**\n\n\
                This response is given when you request an entity or endpoint that does not exists.\
                "
                .to_string(),
                ..Default::default()
            }),
        );
        responses.insert(
            "422".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "\
                **Unprocessable Entity**\n\n\
                This response is given when you request body is not correctly formatted. \
                "
                .to_string(),
                ..Default::default()
            }),
        );
        responses.insert(
            "500".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "\
                **Internal Server Error**\n\n\
                This response is given when something wend wrong on the server. \
                "
                .to_string(),
                ..Default::default()
            }),
        );
        Ok(Responses {
            responses,
            ..Default::default()
        })
    }
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
impl<'r> Responder<'r, 'static> for ServiceError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        // Convert object to json
        let body = serde_json::to_string(&self).unwrap();
        Response::build()
            .sized_body(body.len(), std::io::Cursor::new(body))
            .header(ContentType::JSON)
            .status(self.http_status)
            .ok()
    }
}

#[cfg(feature = "backend")]
impl From<rocket::serde::json::Error<'_>> for ServiceError {
    fn from(err: rocket::serde::json::Error) -> Self {
        use rocket::serde::json::Error::*;
        match err {
            Io(io_error) => ServiceError {
                error: "IO Error".to_owned(),
                message: Some(io_error.to_string()),
                http_status: Status::BadRequest,
            },
            Parse(_raw_data, parse_error) => ServiceError {
                error: "Parse Error".to_owned(),
                message: Some(parse_error.to_string()),
                http_status: Status::BadRequest,
            },
        }
    }
}

#[cfg(feature = "frontend")]
impl From<gloo_net::Error> for ServiceError {
    fn from(err: gloo_net::Error) -> Self {
        Self {
            error: "Request Error".to_owned(),
            message: Some(err.to_string()),
            #[cfg(feature = "backend")]
            http_status: Status::default(),
        }
    }
}
