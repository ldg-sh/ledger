pub fn build_key(user_id: &str, file_id: &str) -> String {
    format!("{}/{}", user_id, file_id)
}
