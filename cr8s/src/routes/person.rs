use rocket::http::Status;
use rocket::{
    response::status::Custom,
    serde::json::{serde_json::json, Json, Value},
};

use crate::repos::person::PersonRepo;
use crate::{models::NewPerson, DbConn};

use super::server_error;

#[rocket::post("/person", format = "json", data = "<new_person>")]
pub async fn create_person(
    new_person: Json<NewPerson>,
    db: DbConn,
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(move |c| {
        PersonRepo::create(c, new_person.into_inner())
            .map(|person| Custom(Status::Created, json!(person)))
            .map_err(|e| server_error(e.into()))
    })
    .await
}
