use rocket::{
    http::Status,
    response::status::{Custom, NoContent},
    serde::json::{json, Json, Value},
};

use crate::{
    models::{NewRustacean, Rustacean},
    repositories::RustaceanRepository,
    rocket_routes::DbConnection,
};

const RUSTACEANS_LIMIT: i64 = 100;

#[rocket::get("/rustaceans?<limit>")]
pub async fn get_rustaceans(db: DbConnection, limit: Option<i64>) -> Result<Value, Custom<Value>> {
    db.run(move |connection| {
        RustaceanRepository::find_multiple(connection, limit.unwrap_or_else(|| RUSTACEANS_LIMIT))
            .map(|rustaceans| json!(rustaceans))
            .map_err(|_error| Custom(Status::InternalServerError, json!("Error")))
    })
    .await
}

#[rocket::get("/rustaceans/<id>")]
pub async fn view_rustacean(id: i32, db: DbConnection) -> Result<Value, Custom<Value>> {
    db.run(move |connection| {
        RustaceanRepository::find(connection, id)
            .map(|rustacean| json!(rustacean))
            .map_err(|_error| Custom(Status::InternalServerError, json!("Error")))
    })
    .await
}

#[rocket::post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub async fn create_rustacean(
    new_rustacean: Json<NewRustacean>,
    db: DbConnection,
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(move |connection| {
        RustaceanRepository::create(connection, new_rustacean.into_inner())
            .map(|rustacean| Custom(Status::Created, json!(rustacean)))
            .map_err(|_error| Custom(Status::InternalServerError, json!("Error")))
    })
    .await
}

#[rocket::put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
pub async fn update_rustacean(
    id: i32,
    rustacean: Json<Rustacean>,
    db: DbConnection,
) -> Result<Value, Custom<Value>> {
    db.run(move |connection| {
        RustaceanRepository::update(connection, id, rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|_error| Custom(Status::InternalServerError, json!("Error")))
    })
    .await
}

#[rocket::delete("/rustaceans/<id>")]
pub async fn delete_rustacean(id: i32, db: DbConnection) -> Result<NoContent, Custom<Value>> {
    db.run(move |connection| {
        RustaceanRepository::delete(connection, id)
            .map(|_| NoContent)
            .map_err(|_error| Custom(Status::InternalServerError, json!("Error")))
    })
    .await
}
