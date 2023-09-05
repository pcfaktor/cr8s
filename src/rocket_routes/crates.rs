use rocket::{
    http::Status,
    response::status::{Custom, NoContent},
    serde::json::{json, Json, Value},
};

use crate::{
    models::{Crate, NewCrate, User},
    repositories::CrateRepository,
    rocket_routes::{DbConnection, EditorUser},
};

use super::server_error;

const CRATES_LIMIT: i64 = 100;

#[rocket::get("/crates?<limit>")]
pub async fn get_crates(
    db: DbConnection,
    limit: Option<i64>,
    _user: User,
) -> Result<Value, Custom<Value>> {
    db.run(move |connection| {
        CrateRepository::find_multiple(connection, limit.unwrap_or_else(|| CRATES_LIMIT))
            .map(|crates| json!(crates))
            .map_err(|e| server_error(e.into()))
    })
    .await
}

#[rocket::get("/crates/<id>")]
pub async fn view_crate(id: i32, db: DbConnection, _user: User) -> Result<Value, Custom<Value>> {
    db.run(move |connection| {
        CrateRepository::find(connection, id)
            .map(|a_crate| json!(a_crate))
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    Custom(Status::NotFound, json!("Crate not found"))
                }
                _ => server_error(e.into()),
            })
    })
    .await
}

#[rocket::post("/crates", format = "json", data = "<new_crate>")]
pub async fn create_crate(
    new_crate: Json<NewCrate>,
    db: DbConnection,
    _user: EditorUser,
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(move |connection| {
        CrateRepository::create(connection, new_crate.into_inner())
            .map(|a_crate| Custom(Status::Created, json!(a_crate)))
            .map_err(|e| server_error(e.into()))
    })
    .await
}

#[rocket::put("/crates/<id>", format = "json", data = "<a_crate>")]
pub async fn update_crate(
    id: i32,
    a_crate: Json<Crate>,
    db: DbConnection,
    _user: EditorUser,
) -> Result<Value, Custom<Value>> {
    db.run(move |connection| {
        CrateRepository::update(connection, id, a_crate.into_inner())
            .map(|a_crate| json!(a_crate))
            .map_err(|e| server_error(e.into()))
    })
    .await
}

#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(
    id: i32,
    db: DbConnection,
    _user: EditorUser,
) -> Result<NoContent, Custom<Value>> {
    db.run(move |connection| {
        CrateRepository::delete(connection, id)
            .map(|_| NoContent)
            .map_err(|e| server_error(e.into()))
    })
    .await
}
