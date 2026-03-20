use sea_orm::prelude::Uuid;
use sea_orm::entity::prelude::*;
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::Set;
use serde::{Deserialize, Serialize};
use crate::refresh_token;

#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub email: String,
    pub github_id: Option<String>,
    pub google_id: Option<String>,
    pub username: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "refresh_token::Entity")]
    RefreshToken,
}

impl Related<refresh_token::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RefreshToken.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let now = chrono::Utc::now().fixed_offset();

        if insert {
            if self.id.is_not_set() {
                self.id = Set(Uuid::new_v4().to_string());
            }
            self.created_at = Set(now);
        }

        self.updated_at = Set(now);

        Ok(self)
    }
}