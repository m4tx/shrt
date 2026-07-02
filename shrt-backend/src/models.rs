use chrono::{DateTime, Utc};
use cot::db::{Auto, model};

#[model]
pub struct Link {
    #[model(primary_key)]
    pub id: Auto<i32>,
    pub slug: String,
    pub url: String,
    pub created_at: DateTime<Utc>,
    pub visits: i32,
}
