use crate::types::error::AppError;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use tracing::info;

#[derive(Clone)]
pub struct PostgresService {
    pub(crate) database_connection: DatabaseConnection,
}

impl PostgresService {
    pub async fn new(uri: &str) -> Result<Self, AppError> {
        let database_connection = Database::connect(uri).await?;
        Migrator::up(&database_connection, None).await?;

        info!("Successfully connected to PostgreSQL and ran migrations.");

        Ok(Self {
            database_connection,
        })
    }
}
