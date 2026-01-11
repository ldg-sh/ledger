use crate::db::postgres_service::PostgresService;
use crate::{
    types::{error::AppError, token::TokenType, user},
    utils::token::{self, encrypt, new_token},
};
use chrono::Utc;
use entity::user::{ActiveModel as UserActive, Entity as User, Model as UserModel};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, Set,
    TransactionTrait,
};
use uuid::Uuid;

impl PostgresService {
    pub async fn user_exists_by_email(&self, email: &str) -> Result<bool, AppError> {
        Ok(User::find()
            .filter(entity::user::Column::Email.eq(email))
            .count(&self.database_connection)
            .await?
            > 0)
    }

    pub async fn get_user_by_id(&self, id: &Uuid) -> Result<UserModel, AppError> {
        Ok(User::find_by_id(*id)
            .one(&self.database_connection)
            .await?
            .ok_or_else(|| DbErr::RecordNotFound("User does not exist".into()))?)
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<UserModel, AppError> {
        Ok(User::find()
            .filter(entity::user::Column::Email.eq(email))
            .one(&self.database_connection)
            .await?
            .ok_or_else(|| DbErr::RecordNotFound("User does not exist".into()))?)
    }

    pub async fn get_user_auth_hash(&self, id: Uuid) -> Result<String, AppError> {
        Ok(self.get_user_by_id(&id).await?.auth_hash)
    }

    pub async fn regenerate_user_token(&self, user_id: &Uuid) -> Result<String, AppError> {
        let user = self.get_user_by_id(user_id).await?;
        let token = new_token(TokenType::User);
        let encrypted = encrypt(&token).map_err(|_| DbErr::RecordNotUpdated)?;
        let mut am: UserActive = user.into();
        am.auth_hash = Set(encrypted);
        am.updated_at = Set(Utc::now());
        am.update(&self.database_connection).await?;
        Ok(token)
    }

    /// Signup: create user.
    pub async fn create_user(&self, payload: user::DBUserCreate) -> Result<Uuid, AppError> {
        if self.user_exists_by_email(&payload.email).await? {
            return Err(AppError::AlreadyExists);
        }
        let uid = token::new_id();
        let now = Utc::now();
        let txn = self.database_connection.begin().await?;

        User::insert(UserActive {
            id: Set(uid),
            name: Set(payload.name),
            email: Set(payload.email),
            auth_hash: Set(payload.auth_hash),
            created_at: Set(now),
            updated_at: Set(now),
        })
        .exec(&txn)
        .await?;

        txn.commit().await?;
        Ok(uid)
    }

    pub async fn update_user_name(&self, user_id: Uuid, name: String) -> Result<(), AppError> {
        let mut am: UserActive = self.get_user_by_id(&user_id).await?.into();
        am.name = Set(name);
        am.updated_at = Set(Utc::now());
        Ok(am.update(&self.database_connection).await.map(|_| ())?)
    }

    pub async fn update_user_email(&self, user_id: Uuid, email: String) -> Result<(), AppError> {
        if self.user_exists_by_email(&email).await? {
            return Err(AppError::Db(DbErr::RecordNotUpdated));
        }
        let mut am: UserActive = self.get_user_by_id(&user_id).await?.into();
        am.email = Set(email);
        am.updated_at = Set(Utc::now());
        Ok(am.update(&self.database_connection).await.map(|_| ())?)
    }

    // Legacy helpers removed: team management no longer exists in the simplified model.
}
