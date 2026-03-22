use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use sea_orm::entity::prelude::*;

#[cfg(not(feature = "ssr"))]
type Uuid = uuid::Uuid;
#[cfg(not(feature = "ssr"))]
type DateTimeWithTimeZone = chrono::DateTime<chrono::FixedOffset>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(DeriveEntityModel))]
#[cfg_attr(feature = "ssr", sea_orm(table_name = "refresh_token"))]
pub struct Model {
    #[cfg_attr(feature = "ssr", sea_orm(primary_key, auto_increment = false))]
    pub id: Uuid,
    pub user_id: String,
    pub token: String,
    pub expires_at: DateTimeWithTimeZone,
    pub created_at: DateTimeWithTimeZone,
}

#[cfg(feature = "ssr")]
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    User,
}

#[cfg(feature = "ssr")]
impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

#[cfg(feature = "ssr")]
impl ActiveModelBehavior for ActiveModel {}