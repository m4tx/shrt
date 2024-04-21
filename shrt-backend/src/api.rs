use rocket::response::status::NoContent;
use rocket::serde::json::Json;
use shrt_common::errors::ServiceError;

pub mod auth;
pub mod links;

type ApiResult<T> = Result<Json<T>, ServiceError>;
type NoContentApiResult = Result<NoContent, ServiceError>;
pub type DataResult<'a, T> = Result<Json<T>, rocket::serde::json::Error<'a>>;
