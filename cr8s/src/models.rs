use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::{
    prelude::{Insertable, Queryable},
    query_builder::AsChangeset,
};
use serde::{Deserialize, Serialize};

#[derive(Queryable, AsChangeset, Deserialize, Serialize)]
pub struct Rustacean {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=rustaceans)]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}

#[derive(Queryable, AsChangeset, Deserialize, Serialize)]
pub struct Person {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=persons)]
pub struct NewPerson {
    pub name: String,
}

#[derive(Queryable, AsChangeset, Deserialize, Serialize)]
pub struct Crate {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=crates)]
pub struct NewCrate {
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}

#[derive(Queryable, AsChangeset, Deserialize, Serialize)]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub username: String,
    pub password: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Queryable, AsChangeset, Deserialize, Serialize)]
pub struct Role {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub code: String,
    pub name: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=roles)]
pub struct NewRole {
    pub code: String,
    pub name: String,
}

#[derive(Queryable, AsChangeset, Deserialize, Serialize)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Role))]
pub struct UserRole {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=user_roles)]
pub struct NewUserRole {
    pub user_id: i32,
    pub role_id: i32,
}
