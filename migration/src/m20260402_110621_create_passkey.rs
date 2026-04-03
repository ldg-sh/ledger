use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Passkey::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Passkey::CredId).string().not_null().primary_key())
                    .col(ColumnDef::new(Passkey::UserId).string().not_null())
                    .col(ColumnDef::new(Passkey::PasskeyData).json().not_null())
                    .col(ColumnDef::new(Passkey::CreatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AuthSession::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AuthSession::UserId).string().not_null().primary_key())
                    .col(ColumnDef::new(AuthSession::StateData).json().not_null())
                    .col(ColumnDef::new(AuthSession::ExpiresAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Passkey::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(AuthSession::Table).to_owned())
            .await
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(DeriveIden)]
enum Passkey {
    Table,
    CredId,
    UserId,
    PasskeyData,
    CreatedAt
}

#[allow(clippy::enum_variant_names)]
#[derive(DeriveIden)]
enum AuthSession {
    Table,
    UserId,
    StateData,
    ExpiresAt
}