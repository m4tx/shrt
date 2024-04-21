use std::error::Error;
use std::fmt::{Display, Formatter};

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{password_hash, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::Utc;
use rocket::http::Status;
use rocket::post;
use rocket_okapi::openapi;
use sea_orm::prelude::DateTimeUtc;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DbConn, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter,
};
use sea_orm_rocket::Connection;
use shrt_common::auth::{ChangePasswordRequest, LoginRequest};
use shrt_common::errors::ServiceError;
use shrt_entity::user;

use crate::api::{ApiResult, DataResult};
use crate::pool::Db;

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct HashedPassword(String);

#[derive(Debug)]
pub enum HashedPasswordError {
    Argon2Error(password_hash::Error),
}

impl Display for HashedPasswordError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HashedPasswordError::Argon2Error(e) => {
                write!(f, "Argon2 error: {}", e)
            }
        }
    }
}

impl Error for HashedPasswordError {}

impl From<password_hash::Error> for HashedPasswordError {
    fn from(value: password_hash::Error) -> Self {
        Self::Argon2Error(value)
    }
}

impl From<HashedPasswordError> for ServiceError {
    fn from(value: HashedPasswordError) -> Self {
        Self {
            error: "Error occurred when hashing the password".to_string(),
            message: Some(value.to_string()),
            http_status: Status::InternalServerError,
        }
    }
}

impl HashedPassword {
    pub fn new_argon2(password: &str) -> Result<Self, HashedPasswordError> {
        let argon2 = Self::argon2_instance();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();
        Ok(Self(password_hash))
    }

    pub fn str(&self) -> &str {
        &self.0
    }

    pub fn verify_password(&self, password: &str) -> Result<bool, HashedPasswordError> {
        let parsed_hash = PasswordHash::new(&self.0)?;
        let result = Self::argon2_instance()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok();
        Ok(result)
    }

    #[must_use]
    fn argon2_instance<'a>() -> Argon2<'a> {
        Argon2::default()
    }
}

#[derive(Debug)]
struct UserDb;

impl UserDb {
    pub async fn create_user(
        db: &DbConn,
        user: user::ActiveModel,
    ) -> Result<user::Model, UserDbError> {
        Ok(user.insert(db).await?)
    }

    pub async fn exists(db: &DbConn, username: &str) -> Result<bool, UserDbError> {
        let count = user::Entity::find()
            .filter(user::Column::Username.contains(username))
            .count(db)
            .await?;
        Ok(count > 0)
    }

    pub async fn find_user_by_username(
        db: &DbConn,
        username: &str,
    ) -> Result<user::Model, UserDbError> {
        user::Entity::find()
            .filter(user::Column::Username.contains(username))
            .one(db)
            .await?
            .ok_or(UserDbError::not_found(username.to_owned()))
    }
}

#[derive(Debug)]
enum UserDbError {
    DbError(DbErr),
    NotFound { username: String },
}

impl UserDbError {
    #[must_use]
    fn not_found(username: String) -> Self {
        Self::NotFound { username }
    }
}

impl From<DbErr> for UserDbError {
    fn from(value: DbErr) -> Self {
        UserDbError::DbError(value)
    }
}

impl From<UserDbError> for ServiceError {
    fn from(value: UserDbError) -> Self {
        match value {
            UserDbError::NotFound { username } => ServiceError {
                error: "User not found".to_string(),
                message: Some(format!("User with username {} not found", username)),
                http_status: Status::NotFound,
            },
            UserDbError::DbError(db_error) => ServiceError {
                error: "Database error".to_string(),
                message: Some(db_error.to_string()),
                http_status: Status::InternalServerError,
            },
        }
    }
}

#[openapi]
#[post("/auth/login", data = "<post_data>")]
pub async fn login(
    conn: Connection<'_, Db>,
    post_data: DataResult<'_, LoginRequest>,
) -> ApiResult<()> {
    unimplemented!()
}

#[openapi]
#[post("/auth/change-password", data = "<post_data>")]
pub async fn change_password(
    conn: Connection<'_, Db>,
    post_data: DataResult<'_, ChangePasswordRequest>,
) -> ApiResult<()> {
    unimplemented!()
}

// TODO login with JWT
// TODO change password
// TODO CLI signup

// async fn login_internal(db: &DbConn, login_request: &LoginRequest) ->
// Result<(), ServiceError> {     let user = UserDb::find_user_by_username(db,
// &login_request.username).await?;     let hashed_password =
// HashedPassword(user.password);     if !hashed_password.verify_password(&
// login_request.password)? {         return Err(ServiceError {
//             error: "Invalid password".to_string(),
//             message: Some("Invalid password".to_string()),
//             http_status: Status::Unauthorized,
//         });
//     }
//     Ok(())
// }

// async fn change_password_internal(
//     db: &DbConn,
//     change_password_request: &ChangePasswordRequest,
// ) -> Result<(), ServiceError> {
//     let user = UserDb::find_user_by_username(db,
// &change_password_request.username).await?;     let hashed_password =
// HashedPassword(user.password);     if !hashed_password.verify_password(&
// change_password_request.old_password)? {         return Err(ServiceError {
//             error: "Invalid password".to_string(),
//             message: Some("Invalid password".to_string()),
//             http_status: Status::Unauthorized,
//         });
//     }
//     let new_hashed_password =
// HashedPassword::new_argon2(&change_password_request.new_password)?;
//     user::Entity::update()
//         .set(user::Column::Password, new_hashed_password.0)
//         .filter(user::Column::Id.eq(user.id))
//         .exec(db)
//         .await?;
//     Ok(())
// }

pub async fn create_user(db: &DbConn, username: &str, password: &str) -> Result<(), ServiceError> {
    if UserDb::exists(db, username).await? {
        return Err(ServiceError {
            error: "User already exists".to_string(),
            message: Some("User already exists".to_string()),
            http_status: Status::Conflict,
        });
    }
    let hashed_password = HashedPassword::new_argon2(password)?;
    let user = user::ActiveModel {
        id: ActiveValue::NotSet,
        username: ActiveValue::Set(username.to_owned()),
        password: ActiveValue::Set(hashed_password.str().to_owned()),
        active: ActiveValue::Set(true),
        created: ActiveValue::Set(Utc::now()),
        last_login: ActiveValue::NotSet,
    };
    UserDb::create_user(db, user).await?;
    Ok(())
}
