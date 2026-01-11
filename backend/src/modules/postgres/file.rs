use sea_orm::{QueryFilter, QueryOrder};
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
use sea_orm::sea_query::Expr;
use crate::types::file::TCreateDirectory;

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

    pub async fn list_related_files(&self, path: &str, user_id: &str) -> AResult<Vec<FileModel>> {
        let mut query = File::find()
            .filter(entity::file::Column::OwnerId.eq(user_id));

        if !path.is_empty() {
            query = query.filter(entity::file::Column::Path.starts_with(path));
        }

        let files = query
            .order_by_asc(entity::file::Column::Path)
            .all(&self.database_connection)
            .await?;

        Ok(files)
    }

    pub async fn list_files(&self, path: &str, user_id: &str) -> AResult<Vec<FileModel>> {
        let files = File::find()
            .filter(entity::file::Column::OwnerId.eq(user_id))
            .filter(entity::file::Column::Path.eq(path))
            .all(&self.database_connection)
            .await?;

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

    pub async fn create_directory(&self, dir: TCreateDirectory) -> Result<String, AppError> {
        let dir_am = FileActiveModel {
            id: Set(dir.id.clone()),
            file_name: Set(dir.file_name),
            upload_id: Set(dir.upload_id),
            owner_id: Set(dir.owner_id),
            file_size: Set(0),
            created_at: Set(dir.created_at),
            upload_completed: Set(true),
            file_type: Set("directory".to_string()),
            path: Set(dir.path),
        };

        File::insert(dir_am)
            .exec(&self.database_connection)
            .await?;

        Ok(dir.id)
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

    pub async fn move_multiple(&self, file_ids: Vec<String>, new_path: &str, user_id: &str) -> Result<(), AppError> {
        File::update_many()
            .col_expr(entity::file::Column::Path, Expr::value(new_path))
            .filter(entity::file::Column::Id.is_in(file_ids))
            .filter(entity::file::Column::OwnerId.eq(user_id))
            .exec(&self.database_connection)
            .await?;

        Ok(())
    }

    pub async fn delete_prefix(&self, source_prefix: &str, user_id: &str) -> Result<(), AppError> {
        File::delete_many()
            .filter(entity::file::Column::Path.starts_with(source_prefix))
            .filter(entity::file::Column::OwnerId.eq(user_id))
            .exec(&self.database_connection)
            .await?;

        Ok(())
    }

    pub async fn move_prefix(&self, old_prefix: &str, new_prefix: &str, user_id: &str) -> Result<(), AppError> {
        File::update_many()
            .filter(entity::file::Column::Path.starts_with(old_prefix))
            .filter(entity::file::Column::OwnerId.eq(user_id))
            .col_expr(
                entity::file::Column::Path,
                Expr::cust_with_exprs(
                    "REPLACE(path, $1, $2)",
                    vec![old_prefix.into(), new_prefix.into()]
                ),
            )
            .exec(&self.database_connection)
            .await?;

        Ok(())
    }

    pub async fn delete_multiple(&self, file_ids: Vec<String>, user_id: &str) -> AResult<()> {
        File::delete_many()
            .filter(entity::file::Column::Id.is_in(file_ids))
            .filter(entity::file::Column::OwnerId.eq(user_id))
            .exec(&self.database_connection)
            .await?;
        Ok(())
    }

    pub async fn create_multiple(&self, files: Vec<TCreateFile>) -> AResult<()> {
        let active_models: Vec<FileActiveModel> = files.into_iter().map(|f| FileActiveModel {
            id: Set(f.id),
            file_name: Set(f.file_name),
            upload_id: Set(f.upload_id),
            owner_id: Set(f.owner_id),
            file_size: Set(f.file_size),
            created_at: Set(f.created_at),
            upload_completed: Set(f.upload_completed),
            file_type: Set(f.file_type),
            path: Set(f.path),
        }).collect();

        File::insert_many(active_models)
            .exec(&self.database_connection)
            .await?;
        Ok(())
    }
}
