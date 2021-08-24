#[macro_use] extern crate rocket;
use server::config;

#[launch]
pub fn rocket() -> _ {
    config()
}