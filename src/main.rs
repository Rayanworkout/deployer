#[macro_use]
extern crate rocket;

mod db;
mod models;
mod routes;

use rocket_dyn_templates::Template;

// FileServer to serve static files
use rocket::fs::FileServer;

use crate::routes::{create_project_view, delete_project_view, home};

#[launch]
fn rocket() -> _ {
    // Before building the app, we make sure
    // the database exists.
    db::create_database().expect("Failed to init database, aborting ...");

    rocket::build()
        .mount("/", routes![home, create_project_view, delete_project_view])
        .attach(Template::fairing())
        .mount("/", FileServer::from("static"))
}
