pub mod crates;
pub mod rustaceans;

use diesel::PgConnection;

#[rocket_sync_db_pools::database("postgres")]
pub struct DbConnection(PgConnection);
