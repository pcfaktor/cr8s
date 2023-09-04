use rocket::{
    http::Status,
    response::status::{Custom, NoContent},
    serde::json::{json, Json, Value},
};

use crate::{
    models::{NewRustacean, Rustacean, User},
    repositories::RustaceanRepository,
    rocket_routes::DbConnection,
};

use super::server_error;

const RUSTACEANS_LIMIT: i64 = 100;

#[rocket::get("/rustaceans?<limit>")]
pub async fn get_rustaceans(
    db: DbConnection,
    limit: Option<i64>,
    _user: User,
) -> Result<Value, Custom<Value>> {
    db.run(move |connection| {
        RustaceanRepository::find_multiple(connection, limit.unwrap_or_else(|| RUSTACEANS_LIMIT))
            .map(|rustaceans| json!(rustaceans))
            .map_err(|e| server_error(e.into()))
    })
    .await
}

#[rocket::get("/rustaceans/<id>")]
pub async fn view_rustacean(
    id: i32,
    db: DbConnection,
    _user: User,
) -> Result<Value, Custom<Value>> {
    db.run(move |connection| {
        RustaceanRepository::find(connection, id)
            .map(|rustacean| json!(rustacean))
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    Custom(Status::NotFound, json!("Rustacean not found"))
                }
                _ => server_error(e.into()),
            })
    })
    .await
}

#[rocket::post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub async fn create_rustacean(
    new_rustacean: Json<NewRustacean>,
    db: DbConnection,
    _user: User,
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(move |connection| {
        RustaceanRepository::create(connection, new_rustacean.into_inner())
            .map(|rustacean| Custom(Status::Created, json!(rustacean)))
            .map_err(|e| server_error(e.into()))
    })
    .await
}

#[rocket::put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
pub async fn update_rustacean(
    id: i32,
    rustacean: Json<Rustacean>,
    db: DbConnection,
    _user: User,
) -> Result<Value, Custom<Value>> {
    db.run(move |connection| {
        RustaceanRepository::update(connection, id, rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|e| server_error(e.into()))
    })
    .await
}

#[rocket::delete("/rustaceans/<id>")]
pub async fn delete_rustacean(
    id: i32,
    db: DbConnection,
    _user: User,
) -> Result<NoContent, Custom<Value>> {
    db.run(move |connection| {
        RustaceanRepository::delete(connection, id)
            .map(|_| NoContent)
            .map_err(|e| server_error(e.into()))
    })
    .await
}
