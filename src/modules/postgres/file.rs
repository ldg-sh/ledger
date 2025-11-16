use crate::{
    modules::postgres::postgres_service::PostgresService,
    types::{error::AppError, file::TCreateFile},
};
use anyhow::Result as AResult;
use entity::file::ActiveModel as FileActiveModel;
use entity::file::{Entity as File, Model as FileModel};
use sea_orm::{EntityTrait, Set};

impl PostgresService {
    pub async fn list_files(&self) -> AResult<Vec<FileModel>> {
        let files = File::find().all(&self.database_connection).await?;

        Ok(files)
    }

    pub async fn create_file(&self, file: TCreateFile) -> Result<String, AppError> {
        let file_am = FileActiveModel {
            id: Set(file.id.clone()),
            file_name: Set(file.file_name),
            upload_id: Set(file.upload_id),
            file_size: Set(file.file_size),
            created_at: Set(file.created_at),
            upload_completed: Set(file.upload_completed),
            file_type: Set(file.file_type),
        };

        File::insert(file_am)
            .exec(&self.database_connection)
            .await?;

        Ok(file.id)
    }
}
