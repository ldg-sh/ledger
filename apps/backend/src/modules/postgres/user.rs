use sea_orm::ColumnTrait;
use crate::authentication::routes::providers::Provider;
use crate::modules::postgres::postgres_service::PostgresService;
use chrono::Utc;

use entity::refresh_token;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{sea_query::OnConflict, EntityTrait, QueryFilter, Set};
use std::io::Error;
use std::io::ErrorKind;
use crate::types::user::User;

impl PostgresService {
    pub async fn upsert_oauth_user(
        &self,
        email: String,
        username: String,
        provider_id: String,
        avatar: Option<String>,
        provider: Provider,
    ) -> Result<String, Error> {
        use entity::user::{ActiveModel, Column, Entity as User};

        let mut active_model = ActiveModel {
            id: Set(uuid::Uuid::new_v4().to_string()),
            email: Set(email),
            username: Set(username),
            created_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
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
                    .to_owned()
            )
            .exec_with_returning(&self.database_connection)
            .await
            .map_err(|e| Error::new(ErrorKind::Other, format!("DB Upsert Error: {}", e)))?;

        println!("{:?}", result);

        Ok(result.id)
    }

    pub async fn store_refresh_token(
        &self,
        user_id: String,
        token: String,
        expires_at: DateTimeWithTimeZone,
    ) -> Result<(), Error> {
        use entity::refresh_token::Entity as RefreshToken;

        let refresh_token = refresh_token::ActiveModel {
            id: Set(uuid::Uuid::new_v4()),
            user_id: Set(user_id),
            token: Set(token),
            expires_at: Set(expires_at),
            created_at: Set(DateTimeWithTimeZone::from(Utc::now())),
        };

        RefreshToken::insert(refresh_token)
            .exec(&self.database_connection)
            .await
            .map_err(|e| Error::new(ErrorKind::Other, format!("DB Insert Error: {}", e)))?;

        Ok(())
    }

    pub async fn delete_refresh_token(&self, refresh_token: String) -> Result<(), Error> {
        use entity::refresh_token::Entity as RefreshToken;

        RefreshToken::delete_many()
            .filter(refresh_token::Column::Token.eq(refresh_token))
            .exec(&self.database_connection)
            .await
            .map_err(|e| Error::new(ErrorKind::Other, format!("DB Delete Error: {}", e)))?;

        Ok(())
    }

    pub async fn get_refresh_token(&self, refresh_token: String) -> Result<refresh_token::Model, Error> {
        use entity::refresh_token::Entity as RefreshToken;

        let res = RefreshToken::find()
            .filter(refresh_token::Column::Token.eq(refresh_token))
            .one(&self.database_connection)
            .await
            .map_err(|e| Error::new(ErrorKind::Other, format!("DB Query Error: {}", e)));

        Ok(res?.ok_or_else(|| Error::new(ErrorKind::NotFound, "Refresh token not found"))?)
    }

    pub async fn get_user_information(&self, user_id: String) -> Result<User, Error> {
        use entity::user::Entity as User;

        let user = User::find()
            .filter(entity::user::Column::Id.eq(user_id))
            .one(&self.database_connection)
            .await
            .map_err(|e| Error::new(ErrorKind::Other, format!("DB Query Error: {}", e)))?
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "User not found"))?;

        Ok(crate::types::user::User::from(user))
    }
}