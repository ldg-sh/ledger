use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(File::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(File::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(File::FileName).string().not_null())
                    .col(
                        ColumnDef::new(File::FileOwnerId)
                            .array(ColumnType::Text)
                            .not_null(),
                    )
                    .col(ColumnDef::new(File::UploadId).string().not_null())
                    .col(ColumnDef::new(File::FileSize).big_unsigned().not_null())
                    .col(
                        ColumnDef::new(File::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(File::UploadCompleted)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(ColumnDef::new(File::FileType).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(File::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(DeriveIden)]
enum File {
    Table,
    Id,
    FileName,
    FileOwnerId,
    UploadId,
    FileSize,
    CreatedAt,
    UploadCompleted,
    FileType,
}
