use sea_orm::entity::prelude::DateTimeUtc;

pub struct TCreateFile {
    pub id: String,
    pub file_name: String,
    pub file_owner_id: Vec<String>,
    pub upload_id: String,
    pub file_size: i64,
    pub created_at: DateTimeUtc,
    pub upload_completed: bool,
    pub file_type: String,
}
