use crate::modules::postgres::postgres_service::PostgresService;
use ledger_backend_entity::file::Entity as FileEntity;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, Set};
use std::io::{Error, ErrorKind};

impl PostgresService {
    pub async fn mark_upload_complete(
        &self,
        file_id: &str,
        upload_id: &str,
        file_size: u64,
    ) -> Result<(), Error> {
        let file = FileEntity::find_by_id(file_id)
            .one(&self.database_connection)
            .await
            .map_err(|e| Error::other(format!("Database query error: {}", e)))?
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "File not found in database"))?;

        let mut file_model = file.into_active_model();
        file_model.upload_id = Set(upload_id.to_string());
        file_model.file_size = Set(file_size as i64);
        file_model.upload_completed = Set(true);

        file_model
            .update(&self.database_connection)
            .await
            .map_err(|e| Error::other(format!("Failed to update file in database: {}", e)))?;

        Ok(())
    }
}
