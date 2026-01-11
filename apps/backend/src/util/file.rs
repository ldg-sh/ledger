use crate::middleware::authentication::AuthenticatedUser;

pub fn build_key(user: &AuthenticatedUser, file_id: &str) -> String {
    format!("{}/{}", user.id, file_id)
}

pub fn is_directory(file_type: &str) -> bool {
    file_type == "directory"
}