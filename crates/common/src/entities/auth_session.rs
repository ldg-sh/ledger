use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use sea_orm::entity::prelude::*;

#[cfg(not(feature = "ssr"))]
type DateTimeWithTimeZone = chrono::DateTime<chrono::FixedOffset>;

#[cfg(not(feature = "ssr"))]
type Json = String;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(DeriveEntityModel))]
#[cfg_attr(feature = "ssr", sea_orm(table_name = "auth_session"))]
pub struct Model {
    #[cfg_attr(feature = "ssr", sea_orm(primary_key, auto_increment = false))]
    pub user_id: String,
    pub state_data: Json,
    pub state_type: String,
    pub expires_at: DateTimeWithTimeZone,
}

#[cfg(feature = "ssr")]
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[cfg(feature = "ssr")]
impl ActiveModelBehavior for ActiveModel {}
