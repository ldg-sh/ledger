use crate::modules::postgres::postgres::PostgresService;
use anyhow::Result;
use entity::file::{Entity as File, Model as FileModel};
use sea_orm::EntityTrait;

impl PostgresService {
    pub async fn list_files(&self) -> Result<Vec<FileModel>> {
        let files = File::find()
            .all(&self.database_connection)
            .await?;

        Ok(files)
    }
}
