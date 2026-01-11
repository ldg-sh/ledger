use uuid::Uuid;

#[allow(dead_code)]
pub fn generate_unique_file_name() -> String {
    let unique = Uuid::new_v4();

    unique.to_string()
}
