use crate::models::User;
use argon2::{
    password_hash::{Error, SaltString},
    PasswordHash, PasswordHasher, PasswordVerifier,
};
use rand::{distributions::Alphanumeric, rngs::OsRng, Rng};

pub const SESSION_LIFE_TIME: usize = 3 * 60 * 60;
pub const SESSION_ID_LENGTH: usize = 128;

#[derive(serde::Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub fn authorize_user(user: &User, credentials: &Credentials) -> Result<String, Error> {
    let db_hash = PasswordHash::new(&user.password)?;
    let argon = argon2::Argon2::default();
    argon.verify_password(credentials.password.as_bytes(), &db_hash)?;

    let sesssion_id = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(SESSION_ID_LENGTH)
        .map(char::from)
        .collect();

    Ok(sesssion_id)
}

pub fn hash_password(password: String) -> Result<String, Error> {
    let salt = SaltString::generate(OsRng);
    let argon = argon2::Argon2::default();
    let password_hash = argon.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}
