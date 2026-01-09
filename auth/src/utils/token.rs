use crate::{db::postgres_service::PostgresService, types::token::TokenType};
use anyhow::Result as AResult;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, prelude::BASE64_STANDARD, Engine as _};
use rand_core::{OsRng, RngCore};
use uuid::Uuid;

pub fn new_id() -> Uuid {
    Uuid::new_v4()
}

pub fn new_nanoid(len: usize) -> String {
    nanoid::nanoid!(len, &nanoid::alphabet::SAFE).to_string()
}

pub fn new_token(token_type: TokenType) -> String {
    let mut buf = [0u8; 32];
    let mut rng = OsRng;
    rng.fill_bytes(&mut buf);
    format!("{}_{}", token_type, URL_SAFE_NO_PAD.encode(buf))
}

pub fn encrypt(token: &str) -> Result<String, argon2::password_hash::Error> {
    let mut rng = OsRng;
    let salt = SaltString::generate(&mut rng);
    let hash = Argon2::default().hash_password(token.as_bytes(), &salt)?;
    Ok(hash.to_string())
}

pub fn verify(token: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed = PasswordHash::new(hash)?;
    Ok(Argon2::default()
        .verify_password(token.as_bytes(), &parsed)
        .is_ok())
}

pub fn encrypt_to_base64(base_string: &str) -> String {
    BASE64_STANDARD.encode(base_string.as_bytes())
}

pub fn decrypt_from_base64(encoded: &str) -> AResult<String> {
    let bytes = BASE64_STANDARD.decode(encoded)?;
    let s = String::from_utf8(bytes)?;
    Ok(s)
}

/// Validates a user token.
///
/// # Arguments
/// * `db` - Reference to the PostgresService used to fetch the stored token.
/// * `b64_token` - A base64-encoded token string in the format `<uuid>.<raw_token>`.
///
/// # Returns
/// `true` if:
/// - the base64 string decodes successfully,
/// - the first part is a valid UUID,
/// - a token for that UUID exists in the database,
/// - and the provided raw token matches the stored encrypted token.
///
/// Otherwise, returns `false`.
///
/// # Example
/// ```ignore
/// let valid = token_valid(&db, "YmFkYmVlZi4xMjM0").await;
/// assert!(!valid);
/// ```
pub async fn token_valid(db: &PostgresService, b64_token: &str) -> bool {
    let token = match decrypt_from_base64(b64_token) {
        Ok(token) => token,
        Err(_) => {
            return false;
        }
    };

    let mut parts = token.split('.');

    let id_str = match parts.next() {
        Some(s) => s,
        None => {
            return false;
        }
    };

    let id = match Uuid::parse_str(id_str) {
        Ok(id) => id,
        Err(_) => {
            return false;
        }
    };

    let encrypted_token = match db.get_user_auth_hash(id).await {
        Ok(encrypted) => encrypted,
        Err(_) => {
            return false;
        }
    };

    let raw_token = match parts.next() {
        Some(token) => token,
        None => {
            return false;
        }
    };

    match verify(raw_token, &encrypted_token) {
        Ok(result) => {
            if !result {
                return false;
            }

            true
        }
        Err(_) => false,
    }
}

/// Extracts the components of a base64-encoded token string.
///
/// A valid token has the form `<uuid>.<raw_token>`, base64-encoded.
/// This function:
/// 1. Decodes the input from base64.
/// 2. Splits it on the `.` character.
/// 3. Parses the first part as a [`Uuid`].
/// 4. Returns the UUID and the second part (`raw_token`) as a `String`.
///
/// # Arguments
/// * `raw_token` - A base64-encoded token string in the format `<uuid>.<raw_token>`.
///
/// # Returns
/// * `Some((Uuid, String))` if decoding and parsing succeed.
/// * `None` if decoding fails, the format is invalid, or the UUID cannot be parsed.
///
/// # Example
/// ```
/// use ledger_auth::utils::token::extract_token_parts;
/// let token = "dXVpZC0xMjM=="; // "<uuid>.secret", base64-encoded
/// if let Some((uid, raw)) = extract_token_parts(token) {
///     println!("id: {uid}, raw: {raw}");
/// }
/// ```
pub fn extract_token_parts(raw_token: &str) -> Option<(Uuid, String)> {
    let decoded_key = match decrypt_from_base64(raw_token) {
        Ok(decoded_key) => decoded_key,
        Err(_) => return None,
    };

    let parts: Vec<&str> = decoded_key.split(".").collect();

    let raw_uid = match parts.first() {
        Some(&uid) => uid,
        None => return None,
    };

    let parsed_uid = match Uuid::parse_str(raw_uid) {
        Ok(uid) => uid,
        Err(_) => return None,
    };

    let key = match parts.get(1) {
        Some(&key) => key,
        None => return None,
    };

    Some((parsed_uid, key.to_owned()))
}

pub fn construct_token(user_id: &Uuid, api_key: &str) -> String {
    encrypt_to_base64(&format!("{user_id}.{api_key}"))
}
