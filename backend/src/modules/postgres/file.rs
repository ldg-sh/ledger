use sea_orm::QueryFilter;
use sea_orm::ColumnTrait;
use crate::{
    modules::postgres::postgres_service::PostgresService,
    types::{error::AppError, file::TCreateFile},
};
use anyhow::Result as AResult;
use chrono::Utc;
use entity::file::ActiveModel as FileActiveModel;
use entity::file::{Entity as File, Model as FileModel};
use sea_orm::{EntityTrait, Set};

impl PostgresService {
    pub async fn get_file(&self, file_id: &str, user_id: &str) -> AResult<Option<FileModel>> {
        let file = File::find()
            .filter(entity::file::Column::Id.eq(file_id))
            .filter(entity::file::Column::OwnerId.eq(user_id))
            .one(&self.database_connection)
            .await?;

        Ok(file)
    }
    
    pub async fn get_multiple_files(&self, file_ids: Vec<&str>, user_id: &str) -> AResult<Vec<FileModel>> {
        let files = File::find()
            .filter(entity::file::Column::Id.is_in(file_ids))
            .filter(entity::file::Column::OwnerId.eq(user_id))
            .all(&self.database_connection)
            .await?;

        Ok(files)
    }
    
    pub async fn list_files(&self, path: &str, user_id: &str) -> AResult<Vec<FileModel>> {
        let files = File::find()
            .filter(entity::file::Column::Path.eq(path))
            .filter(entity::file::Column::OwnerId.eq(user_id))
            .all(&self.database_connection)
            .await?;

        Ok(files)
    }

    pub async fn list_related_files(&self, path: &str, user_id: &str) -> AResult<Vec<FileModel>> {
        let files = if path.is_empty() {
            println!(
                "Listing all files for user_id: {} since path is empty",
                user_id
            );
            File::find()
                .filter(entity::file::Column::OwnerId.eq(user_id))
                .all(&self.database_connection)
                .await?
        } else {
            File::find()
                .filter(entity::file::Column::Path.starts_with(path))
                .filter(entity::file::Column::OwnerId.eq(user_id))
                .all(&self.database_connection)
                .await?
        };

        Ok(files)
    }

    pub async fn create_file(&self, file: TCreateFile) -> Result<String, AppError> {
        let file_am = FileActiveModel {
            id: Set(file.id.clone()),
            file_name: Set(file.file_name),
            upload_id: Set(file.upload_id),
            owner_id: Set(file.owner_id),
            file_size: Set(file.file_size),
            created_at: Set(file.created_at),
            upload_completed: Set(file.upload_completed),
            file_type: Set(file.file_type),
            path: Set(file.path),
        };

        File::insert(file_am)
            .exec(&self.database_connection)
            .await?;

        Ok(file.id)
    }

    pub async fn copy_file(&self, source_file: FileModel, new_file_id: &str, new_path: &str) -> Result<(), AppError> {
        let new_file_am = FileActiveModel {
            id: Set(new_file_id.to_string()),
            file_name: Set(source_file.file_name),
            upload_id: Set(source_file.upload_id),
            owner_id: Set(source_file.owner_id),
            file_size: Set(source_file.file_size),
            created_at: Set(Utc::now()),
            upload_completed: Set(source_file.upload_completed),
            file_type: Set(source_file.file_type),
            path: Set(new_path.to_string()),
        };

        File::insert(new_file_am)
            .exec(&self.database_connection)
            .await?;
        Ok(())
    }

    pub async fn delete_file(&self, source_file: FileModel) -> Result<(), AppError> {
        let file_am: FileActiveModel = source_file.into();

        File::delete(file_am)
            .exec(&self.database_connection)
            .await?;

        Ok(())
    }

    pub async fn rename_file(&self, source_file: FileModel, new_file_name: &str) -> Result<(), AppError> {
        let mut file_am: FileActiveModel = source_file.into();
        file_am.file_name = Set(new_file_name.to_string());

        File::update(file_am)
            .exec(&self.database_connection)
            .await?;

        Ok(())
    }

    pub async fn move_file(&self, source_file: FileModel, new_path: &str) -> Result<(), AppError> {
        let mut file_am: FileActiveModel = source_file.into();
        file_am.path = Set(new_path.to_string());

        File::update(file_am)
            .exec(&self.database_connection)
            .await?;

        Ok(())
    }

    pub async fn copy_multiple(&self, source_files: Vec<FileModel>, new_path: &str) -> Result<(), AppError> {
        for source_file in source_files {
            let new_file_id = uuid::Uuid::new_v4().to_string();

            let new_file_am = FileActiveModel {
                id: Set(new_file_id),
                file_name: Set(source_file.file_name),
                upload_id: Set(source_file.upload_id),
                owner_id: Set(source_file.owner_id),
                file_size: Set(source_file.file_size),
                created_at: Set(Utc::now()),
                upload_completed: Set(source_file.upload_completed),
                file_type: Set(source_file.file_type),
                path: Set(new_path.to_string()),
            };
            File::insert(new_file_am)
                .exec(&self.database_connection)
                .await?;
        }

        Ok(())
    }

    pub async fn delete_multiple(&self, source_files: Vec<FileModel>) -> Result<(), AppError> {
        for file in source_files {
            let file_am: FileActiveModel = file.into();

            File::delete(file_am)
                .exec(&self.database_connection)
                .await?;
        }

        Ok(())
    }

    pub async fn move_multiple(&self, source_files: Vec<FileModel>, new_path: &str) -> Result<(), AppError> {
        for file in source_files {
            let mut file_am: FileActiveModel = file.into();
            file_am.path = Set(new_path.to_string());

            File::update(file_am)
                .exec(&self.database_connection)
                .await?;
        }

        Ok(())
    }
}
