use rocket::{
    http::Status,
    response::status::{Custom, NoContent},
    serde::json::{json, Json, Value},
};

use crate::{
    models::{Crate, NewCrate},
    repositories::CrateRepository,
    rocket_routes::DbConnection,
};

const CRATES_LIMIT: i64 = 100;

#[rocket::get("/crates")]
pub async fn get_crates(db: DbConnection) -> Result<Value, Custom<Value>> {
    db.run(|connection| {
        CrateRepository::find_multiple(connection, CRATES_LIMIT)
            .map(|crates| json!(crates))
            .map_err(|_error| Custom(Status::InternalServerError, json!("Error")))
    })
    .await
}

#[rocket::get("/crates/<id>")]
pub async fn view_crate(id: i32, db: DbConnection) -> Result<Value, Custom<Value>> {
    db.run(move |connection| {
        CrateRepository::find(connection, id)
            .map(|a_crate| json!(a_crate))
            .map_err(|_error| Custom(Status::InternalServerError, json!("Error")))
    })
    .await
}

#[rocket::post("/crates", format = "json", data = "<new_crate>")]
pub async fn create_crate(
    new_crate: Json<NewCrate>,
    db: DbConnection,
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(move |connection| {
        CrateRepository::create(connection, new_crate.into_inner())
            .map(|a_crate| Custom(Status::Created, json!(a_crate)))
            .map_err(|_error| Custom(Status::InternalServerError, json!("Error")))
    })
    .await
}

#[rocket::put("/crates/<id>", format = "json", data = "<a_crate>")]
pub async fn update_crate(
    id: i32,
    a_crate: Json<Crate>,
    db: DbConnection,
) -> Result<Value, Custom<Value>> {
    db.run(move |connection| {
        CrateRepository::update(connection, id, a_crate.into_inner())
            .map(|a_crate| json!(a_crate))
            .map_err(|_error| Custom(Status::InternalServerError, json!("Error")))
    })
    .await
}

#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(id: i32, db: DbConnection) -> Result<NoContent, Custom<Value>> {
    db.run(move |connection| {
        CrateRepository::delete(connection, id)
            .map(|_| NoContent)
            .map_err(|_error| Custom(Status::InternalServerError, json!("Error")))
    })
    .await
}
