use crate::middleware::authentication::AuthenticatedUser;

pub fn build_path(user: &AuthenticatedUser, requested_path: Option<&str>) -> String {
    let mut path = String::new();

    path.push_str(&user.id);

    if let Some(requested_path) = requested_path {
        let trimmed_path = requested_path.trim_matches('/');

        if !trimmed_path.is_empty() {
            path.push('/');
            path.push_str(trimmed_path);
        }
    }

    path
}

pub fn build_key(user: &AuthenticatedUser, requested_path: Option<&str>, file_id: &str) -> String {
    let base_path = build_path(user, requested_path);
    format!("{}/{}", base_path, file_id)
}

pub fn build_key_from_path(user: &AuthenticatedUser, full_path: &str) -> String {
    let trimmed_path = full_path.trim_matches('/');

    format!("{}/{}", user.id, trimmed_path)
}

pub fn extract_file_id_from_key(user: &AuthenticatedUser, key: &str) -> Option<String> {
    let prefix = format!("{}/", user.id);
    if key.starts_with(&prefix) {
        let parts: Vec<&str> = key[prefix.len()..].rsplitn(2, '/').collect();
        if let Some(file_id) = parts.first() {
            return Some(file_id.to_string());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::middleware::authentication::AuthenticatedUser;
    use uuid::Uuid;
    #[test]
    fn test_build_path() {
        let user = AuthenticatedUser {
            id: Uuid::new_v4().to_string(),
        };

        let path = build_path(&user, Some("folder/subfolder"));
        assert_eq!(path, format!("{}/folder/subfolder", user.id));
        let path_no_requested = build_path(&user, None);
        assert_eq!(path_no_requested, user.id);
        let path_with_slashes = build_path(&user, Some("/folder/subfolder/"));
        assert_eq!(path_with_slashes, format!("{}/folder/subfolder", user.id));
        let path_empty_requested = build_path(&user, Some(""));
        assert_eq!(path_empty_requested, user.id);
    }
}