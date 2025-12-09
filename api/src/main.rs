mod controller;

use crate::controller::*;

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![index, rickroll])
}
