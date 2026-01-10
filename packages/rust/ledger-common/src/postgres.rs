use crate::error::AppError;
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use tracing::info;

pub struct PostgresService {
    pub connection: DatabaseConnection,
}

impl PostgresService {
    pub async fn new<M: MigratorTrait>(uri: &str) -> Result<Self, AppError> {
        info!("Connecting to PostgreSQL...");
        let connection = Database::connect(uri).await?;

        info!("Running migrations...");
        M::up(&connection, None).await?;

        info!("Successfully connected to PostgreSQL.");

        Ok(Self { connection })
    }

    pub async fn ping(&self) -> Result<(), sea_orm::DbErr> {
        self.connection.ping().await
    }
}
