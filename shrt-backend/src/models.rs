use chrono::{DateTime, Utc};
use cot::db::{model, Auto};

#[model]
pub struct Link {
    #[model(primary_key)]
    pub id: Auto<i32>,
    pub slug: String,
    pub url: String,
    pub created_at: DateTime<Utc>,
    pub visits: i32,
}
