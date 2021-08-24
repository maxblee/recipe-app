#[macro_use] extern crate rocket;

mod api;
use api::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
}