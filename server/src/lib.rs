#[macro_use] extern crate rocket;
use rocket::{Rocket, Build};

mod api;
use api::*;

pub fn config() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index])
}