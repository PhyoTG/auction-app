// fn main() {
//     println!("Hello, world!");
// }
//  rust-admin // ru$t-@dmin

mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;
use rocket::{get, http::Status, serde::json::Json};

#[get("/")]
fn startFn()-> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello World!! ")))
}

#[launch]
fn rocket()-> _ {
    rocket::build().mount("/", routes![startFn])
}