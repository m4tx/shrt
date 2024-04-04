use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "link")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub slug: String,
    pub url: String,
    #[sea_orm(indexed)]
    pub created_at: DateTimeUtc,
    pub visits: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
