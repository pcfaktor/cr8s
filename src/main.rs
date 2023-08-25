use diesel::PgConnection;

mod models;
mod repositories;
mod rocket_routes;
mod schema;

#[rocket_sync_db_pools::database("postgres")]
pub struct DbConnection(PgConnection);

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            rocket::routes![
                rocket_routes::rustaceans::get_rustaceans,
                rocket_routes::rustaceans::view_rustacean,
                rocket_routes::rustaceans::create_rustacean,
                rocket_routes::rustaceans::update_rustacean,
                rocket_routes::rustaceans::delete_rustacean,
            ],
        )
        .attach(DbConnection::fairing())
        .launch()
        .await;
}
