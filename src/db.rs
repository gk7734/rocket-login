pub mod connection;
pub mod models;

use rocket::fairing::AdHoc;

pub fn init() -> AdHoc {
    AdHoc::on_ignite("Database", |rocket| async {
        let db = connection::establish_connection().await;
        rocket.manage(db)
    })
}