use crate::routes::DbConn;

mod models;
mod schema;
mod repositories;
mod routes;

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

            routes::crates::get_crates,
            routes::crates::view_crate,
            routes::crates::delete_crate,
            routes::crates::create_crate,
            routes::crates::update_crate,
        ])
        .attach(DbConn::fairing())
        .launch()
        .await;
}
