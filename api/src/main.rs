mod controller;
mod models;

use crate::controller::*;
use std::sync::Arc;

#[macro_use] extern crate rocket;

/// Load Real Book data from JSON file
fn load_realbook_data() -> Vec<models::RealBookEntry> {
    let data = std::fs::read_to_string("api/resources/realbook.json")
        .expect("Failed to read realbook.json");
    serde_json::from_str(&data).expect("Failed to parse realbook.json")
}

#[launch]
fn rocket() -> _ {
    let realbook_data = Arc::new(load_realbook_data());

    rocket::build()
        .manage(realbook_data)
        .mount("/api", routes![index, rickroll, search, volumes, random])
}
