use super::{server_error, DbConnection};
use crate::repositories::UserRepository;
use argon2::{PasswordHash, PasswordVerifier};
use rocket::{
    response::status::Custom,
    serde::json::{serde_json::json, Json, Value},
};

#[derive(serde::Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    credentials: Json<Credentials>,
    db: DbConnection,
) -> Result<Value, Custom<Value>> {
    db.run(move |connection| {
        UserRepository::find_by_username(connection, &credentials.username)
            .map(|user| {
                let db_hash = PasswordHash::new(&user.password).unwrap();
                let argon = argon2::Argon2::default();
                if argon
                    .verify_password(credentials.password.as_bytes(), &db_hash)
                    .is_ok()
                {
                    return json!("Success");
                }
                json!("Unauthorized")
            })
            .map_err(|e| server_error(e.into()))
    })
    .await
}
