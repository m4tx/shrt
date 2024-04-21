#[cfg(feature = "backend")]
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// TODO password newtype

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(JsonSchema))]
#[cfg_attr(feature = "backend", serde(crate = "rocket::serde"))]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(JsonSchema))]
#[cfg_attr(feature = "backend", serde(crate = "rocket::serde"))]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}
