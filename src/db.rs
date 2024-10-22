pub mod connection;
pub mod models;

use diesel::r2d2::{ self, ConnectionManager };
use diesel::PgConnection;
use rocket_sync_db_pools::database;

#[database("diesel_postgres_pool")]
pub struct DbConn(r2d2::Pool<ConnectionManager<PgConnection>>);