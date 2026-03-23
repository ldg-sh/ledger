use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use sea_orm::entity::prelude::*;

#[cfg(not(feature = "ssr"))]
type DateTimeWithTimeZone = chrono::DateTime<chrono::FixedOffset>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(DeriveEntityModel))]
#[cfg_attr(feature = "ssr", sea_orm(table_name = "user"))]
pub struct Model {
    #[cfg_attr(feature = "ssr", sea_orm(primary_key, auto_increment = false))]
    pub id: String,
    #[cfg_attr(feature = "ssr", sea_orm(unique))]
    pub email: String,
    #[cfg_attr(feature = "ssr", sea_orm(unique))]
    pub github_id: Option<String>,
    #[cfg_attr(feature = "ssr", sea_orm(unique))]
    pub google_id: Option<String>,
    pub username: String,
    pub avatar_url: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: Option<DateTimeWithTimeZone>,
}

#[cfg(feature = "ssr")]
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::refresh_token::Entity")]
    RefreshToken,
}

#[cfg(feature = "ssr")]
impl Related<super::refresh_token::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RefreshToken.def()
    }
}

#[cfg(feature = "ssr")]
impl ActiveModelBehavior for ActiveModel {}