#[macro_use]
extern crate rocket;

mod db;
mod models;

// Template engine
use rocket_dyn_templates::{Template, context};

// FileServer to serve static files
use rocket::fs::FileServer;

use crate::db::insert_project;
use crate::models::Project;

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[launch]
fn rocket() -> _ {
    // Before building the app, we make sure
    // the database exists.
    db::create_database().expect("Failed to init database, aborting ...");

    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .mount("/", FileServer::from("static"))
}
