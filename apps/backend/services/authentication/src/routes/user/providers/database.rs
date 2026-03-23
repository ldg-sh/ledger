use crate::routes::user::providers::Provider;
use common::entities::prelude::{RefreshToken, User};
use common::entities::user::{ActiveModel, Column};
use common::entities::{refresh_token, user};
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::prelude::{async_trait, DateTimeWithTimeZone};
use sea_orm::sea_query::OnConflict;
use sea_orm::sea_query::prelude::Utc;
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use std::io::Error;
use std::io::ErrorKind;

#[async_trait::async_trait]
pub trait ProviderExtension {
    async fn upsert_oauth_user(
        &self,
        email: String,
        username: String,
        provider_id: String,
        avatar: Option<String>,
        provider: Provider,
    ) -> Result<String, Error>;
    async fn store_refresh_token(
        &self,
        user_id: String,
        token: String,
        expires_at: DateTimeWithTimeZone,
    ) -> Result<(), Error>;
    async fn delete_refresh_token(&self, refresh_token: String) -> Result<(), Error>;
    async fn get_refresh_token(&self, refresh_token: String) -> Result<refresh_token::Model, Error>;
    async fn expire_refresh_tokens(&self) -> Result<(), Error>;
    async fn get_user_information(&self, user_id: String) -> Result<user::Model, Error>;
}

#[async_trait::async_trait]
impl ProviderExtension for DatabaseConnection {
    async fn upsert_oauth_user(
        &self,
        email: String,
        username: String,
        provider_id: String,
        avatar: Option<String>,
        provider: Provider,
    ) -> Result<String, Error> {
        let mut active_model = ActiveModel {
            id: Set(uuid::Uuid::new_v4().to_string()),
            email: Set(email.to_owned()),
            username: Set(username),
            created_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            updated_at: Set(Some(DateTimeWithTimeZone::from(Utc::now()))),
            avatar_url: Set(avatar),
            ..Default::default()
        };

        let conflict_column = match provider {
            Provider::GitHub => {
                active_model.github_id = Set(Some(provider_id));
                Column::GithubId
            }
            Provider::Google => {
                active_model.google_id = Set(Some(provider_id));
                Column::GoogleId
            }
        };

        let result = User::insert(active_model)
            .on_conflict(
                OnConflict::column(conflict_column)
                    .update_columns([Column::Email, Column::UpdatedAt])
                    .to_owned(),
            )
            .exec_with_returning(self)
            .await
            .map_err(|e| Error::new(ErrorKind::Other, format!("DB Upsert Error: {}", e)))?;

        Ok(result.id)
    }

    async fn store_refresh_token(
        &self,
        user_id: String,
        token: String,
        expires_at: DateTimeWithTimeZone,
    ) -> Result<(), Error> {
        let refresh_token = refresh_token::ActiveModel {
            id: Set(uuid::Uuid::new_v4()),
            user_id: Set(user_id),
            token: Set(token),
            expires_at: Set(expires_at),
            created_at: Set(DateTimeWithTimeZone::from(Utc::now())),
        };

        RefreshToken::insert(refresh_token)
            .exec(self)
            .await
            .map_err(|e| Error::new(ErrorKind::Other, format!("DB Insert Error: {}", e)))?;

        Ok(())
    }

    async fn delete_refresh_token(&self, refresh_token: String) -> Result<(), Error> {
        RefreshToken::delete_many()
            .filter(refresh_token::Column::Token.eq(refresh_token))
            .exec(self)
            .await
            .map_err(|e| Error::new(ErrorKind::Other, format!("DB Delete Error: {}", e)))?;

        Ok(())
    }

    async fn get_refresh_token(
        &self,
        refresh_token: String,
    ) -> Result<refresh_token::Model, Error> {
        let res = RefreshToken::find()
            .filter(refresh_token::Column::Token.eq(refresh_token))
            .one(self)
            .await
            .map_err(|e| Error::new(ErrorKind::Other, format!("DB Query Error: {}", e)));

        Ok(res?.ok_or_else(|| Error::new(ErrorKind::NotFound, "Refresh token not found"))?)
    }

    async fn expire_refresh_tokens(&self) -> Result<(), Error> {
        RefreshToken::delete_many()
            .filter(refresh_token::Column::ExpiresAt.lte(DateTimeWithTimeZone::from(Utc::now())))
            .exec(self)
            .await
            .map_err(|e| Error::new(ErrorKind::Other, format!("DB Cleanup Error: {}", e)))?;

        Ok(())
    }

    async fn get_user_information(&self, user_id: String) -> Result<user::Model, Error> {
        let user = User::find()
            .filter(Column::Id.eq(user_id))
            .one(self)
            .await
            .map_err(|e| Error::new(ErrorKind::Other, format!("DB Query Error: {}", e)))?
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "User not found"))?;

        Ok(user)
    }
}
