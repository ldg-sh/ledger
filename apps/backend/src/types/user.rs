use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub github_id: Option<String>,
    pub google_id: Option<String>,
    pub username: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub avatar_url: Option<String>,
}

impl From<entity::user::Model> for User {
    fn from(model: entity::user::Model) -> Self {
        Self {
            id: model.id,
            email: model.email,
            github_id: model.github_id,
            google_id: model.google_id,
            username: model.username,
            created_at: model.created_at,
            updated_at: model.updated_at,
            avatar_url: model.avatar_url,
        }
    }
}