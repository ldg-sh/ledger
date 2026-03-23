#[cfg(feature = "ssr")]
pub use super::file::Entity as File;
#[cfg(feature = "ssr")]
pub use super::refresh_token::Entity as RefreshToken;
#[cfg(feature = "ssr")]
pub use super::user::Entity as User;

pub use super::file::Model as FileModel;
pub use super::refresh_token::Model as RefreshTokenModel;
pub use super::user::Model as UserModel;