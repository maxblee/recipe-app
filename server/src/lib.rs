#[macro_use]
extern crate rocket;
use rocket::{Build, Rocket};

mod api;
use api::*;

pub fn config() -> Rocket<Build> {
    rocket::build().mount("/", routes![index])
}
