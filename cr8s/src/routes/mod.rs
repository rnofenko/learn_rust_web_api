use diesel::PgConnection;

pub mod crates;
pub mod person;
pub mod rustaceans;

#[rocket_sync_db_pools::database("postgres")]
pub struct DbConn(PgConnection);
