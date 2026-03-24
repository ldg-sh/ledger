use serde::{Deserialize, Serialize};
use chrono::{DateTime, FixedOffset};

#[cfg(feature = "ssr")]
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(DeriveEntityModel))]
#[cfg_attr(feature = "ssr", sea_orm(table_name = "file"))]
#[derive(ts_rs::TS)]
#[ts(export, rename = "File", export_to = "File.ts")]
pub struct Model {
    #[cfg_attr(feature = "ssr", sea_orm(primary_key, auto_increment = false))]
    pub id: String,
    pub file_name: String,
    pub owner_id: String,
    pub file_size: i64,
    #[ts(type = "string")]
    pub created_at: DateTime<FixedOffset>,
    pub upload_completed: bool,
    pub file_type: String,
    pub path: String,
}

#[cfg(feature = "ssr")]
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[cfg(feature = "ssr")]
impl ActiveModelBehavior for ActiveModel {}