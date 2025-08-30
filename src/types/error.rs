
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("User already exists")]
    AlreadyExists,
    #[error(transparent)]
    Db(#[from] sea_orm::DbErr),
}
