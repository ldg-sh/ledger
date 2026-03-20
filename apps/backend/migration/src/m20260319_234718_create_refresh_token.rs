use sea_orm_migration::{prelude::*};
use crate::m20260319_222620_create_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RefreshToken::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(RefreshToken::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(RefreshToken::UserId).string().not_null())
                    .col(ColumnDef::new(RefreshToken::Token).string().not_null())
                    .col(ColumnDef::new(RefreshToken::ExpiresAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(RefreshToken::CreatedAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-refresh_token-user_id")
                            .from(RefreshToken::Table, RefreshToken::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
        .drop_table(
            Table::drop()
                .table(RefreshToken::Table)
                .to_owned(),
        ).await
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(DeriveIden)]
enum RefreshToken {
    Table,
    Id,
    UserId,
    Token,
    ExpiresAt,
    CreatedAt
}