mod db;
mod fairings;
mod guards;
mod models;
mod routes;

#[macro_use] extern crate rocket;

use std::io;
use tokio::time::{sleep, Duration };

#[get("/hello")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/world")]
fn world() -> &'static str {
    "Hello"
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {name}!")
}

#[get("/foo/<_..>/bar")]
fn foo_bar() -> &'static str {
    "Foo________Bar"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index, world, delay, hello, foo_bar])
        .launch()
        .await?;
    Ok(())
}
