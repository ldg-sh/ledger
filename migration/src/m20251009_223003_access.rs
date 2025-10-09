use sea_orm_migration::{
    prelude::*,
    sea_orm::{ConnectionTrait, Statement},
    sea_query::Expr,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(File::Table)
                    .add_column(
                        ColumnDef::new(File::OwningTeam)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .add_column(
                        ColumnDef::new(File::AccessIds)
                            .array(ColumnType::Text)
                            .not_null()
                            .default(Expr::cust("'{}'::text[]")),
                    )
                    .to_owned(),
            )
            .await?;

        let db_backend = manager.get_database_backend();
        let db = manager.get_connection();

        db.execute(Statement::from_string(
            db_backend,
            "UPDATE file SET owning_team = file_owner_id[1], access_ids = file_owner_id"
                .to_string(),
        ))
        .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(File::Table)
                    .drop_column(File::FileOwnerId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(File::Table)
                    .add_column(
                        ColumnDef::new(File::FileOwnerId)
                            .array(ColumnType::Text)
                            .not_null()
                            .default(Expr::cust("'{}'::text[]")),
                    )
                    .to_owned(),
            )
            .await?;

        let db_backend = manager.get_database_backend();
        let db = manager.get_connection();

        db.execute(Statement::from_string(
            db_backend,
            "UPDATE file SET file_owner_id = access_ids".to_string(),
        ))
        .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(File::Table)
                    .drop_column(File::OwningTeam)
                    .drop_column(File::AccessIds)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum File {
    Table,
    FileOwnerId,
    OwningTeam,
    AccessIds,
}
