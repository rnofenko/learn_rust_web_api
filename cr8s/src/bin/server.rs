extern crate cr8s;

#[rocket::main]
async fn main() {
    println!("Hello, world!");
    let _ = rocket::build()
        .mount(
            "/",
            rocket::routes![
                cr8s::routes::rustaceans::get_rustaceans,
                cr8s::routes::rustaceans::view_rustacean,
                cr8s::routes::rustaceans::delete_rustacean,
                cr8s::routes::rustaceans::create_rustacean,
                cr8s::routes::rustaceans::update_rustacean,
                cr8s::routes::crates::get_crates,
                cr8s::routes::crates::view_crate,
                cr8s::routes::crates::delete_crate,
                cr8s::routes::crates::create_crate,
                cr8s::routes::crates::update_crate,
                cr8s::routes::person::create_person,
            ],
        )
        .attach(cr8s::routes::DbConn::fairing())
        .launch()
        .await;
}
