#[cfg(feature = "ssr")]
use sea_orm::{FromQueryResult};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "ssr", derive(FromQueryResult))]
pub struct ExplodedItem {
    pub id: String,
    pub file_name: String,
    pub virtual_path: String,
}

#[derive(Deserialize)]
pub struct ExplodeRequest {
    pub item_ids: Vec<String>,
}