use log::info;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection, DbErr};

pub struct PostgresService {
    pub database_connection: DatabaseConnection,
}

impl PostgresService {
    pub async fn new(uri: &str) -> Result<Self, DbErr> {
        info!("Connecting to PostgreSQL...");
        let db = Database::connect(uri).await?;

        info!("Running migrations...");
        Migrator::up(&db, None).await?;

        info!("Connected to PostgreSQL.");
        Ok(Self {
            database_connection: db,
        })
    }
}
