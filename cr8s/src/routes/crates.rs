use rocket::http::Status;
use rocket::{
    response::status::{Custom, NoContent},
    serde::json::{serde_json::json, Json, Value},
};

use crate::{
    models::{Crate, NewCrate},
    repositories::CrateRepository,
    DbConn,
};

use super::server_error;

#[rocket::get("/crates")]
pub async fn get_crates(db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::find_multiple(c, 100)
            .map(|crates| json!(crates))
            .map_err(|e| server_error(e.into()))
    })
    .await
}

#[rocket::get("/crates/<id>")]
pub async fn view_crate(id: i32, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::find(c, id)
            .map(|crat| json!(crat))
            .map_err(|e| server_error(e.into()))
    })
    .await
}

#[rocket::post("/crates", format = "json", data = "<new_crate>")]
pub async fn create_crate(
    new_crate: Json<NewCrate>,
    db: DbConn,
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::create(c, new_crate.into_inner())
            .map(|crat| Custom(Status::Created, json!(crat)))
            .map_err(|e| server_error(e.into()))
    })
    .await
}

#[rocket::put("/crates/<id>", format = "json", data = "<crat>")]
pub async fn update_crate(id: i32, crat: Json<Crate>, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::update(c, id, crat.into_inner())
            .map(|crat| json!(crat))
            .map_err(|e| server_error(e.into()))
    })
    .await
}

#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(id: i32, db: DbConn) -> Result<NoContent, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::delete(c, id)
            .map(|_| NoContent)
            .map_err(|e| server_error(e.into()))
    })
    .await
}
