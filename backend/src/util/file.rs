use crate::middleware::authentication::AuthenticatedUser;

pub fn build_key(user: &AuthenticatedUser, file_id: &str) -> String {
    format!("{}/{}", user.id, file_id)
}