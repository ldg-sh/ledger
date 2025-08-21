use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "file")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub file_name: String,
    pub file_owner_id: String,
    pub upload_id: String,
    pub file_size: i64,
    pub created_at: DateTimeUtc,
    pub upload_completed: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}