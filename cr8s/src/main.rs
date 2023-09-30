use diesel::PgConnection;

mod models;
mod schema;
mod repositories;
mod routes;

#[rocket_sync_db_pools::database("postgres")]
pub struct  DbConn(PgConnection);

#[rocket::main]
async fn main() {
    println!("Hello, world!");
    let _ = rocket::build()
        .mount("/", rocket::routes![
            routes::rustaceans::get_rustaceans,
            routes::rustaceans::view_rustacean,
            routes::rustaceans::delete_rustacean,
            routes::rustaceans::create_rustacean,
            routes::rustaceans::update_rustacean,
        ])
        .attach(DbConn::fairing())
        .launch()
        .await;
}
