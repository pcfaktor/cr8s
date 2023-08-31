use super::{server_error, DbConnection};
use crate::{
    auth::{self, Credentials},
    repositories::UserRepository,
};
use rocket::{
    response::status::Custom,
    serde::json::{serde_json::json, Json, Value},
};

#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    credentials: Json<Credentials>,
    db: DbConnection,
) -> Result<Value, Custom<Value>> {
    db.run(move |connection| {
        UserRepository::find_by_username(connection, &credentials.username)
            .map(|user| {
                if let Ok(token) = auth::authorize_user(&user, &credentials) {
                    return json!(token);
                }
                json!("Unauthorized")
            })
            .map_err(|e| server_error(e.into()))
    })
    .await
}
