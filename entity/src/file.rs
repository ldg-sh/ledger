use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "file")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub file_name: String,
    pub owning_team: String,
    pub access_ids: Vec<String>,
    pub upload_id: String,
    pub file_size: i64,
    pub created_at: DateTimeUtc,
    pub upload_completed: bool,
    pub file_type: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
