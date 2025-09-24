#[macro_use]
extern crate rocket;

mod db;
mod models;
mod routes;

use rocket_dyn_templates::Template;

// FileServer to serve static files
use rocket::fs::FileServer;

use crate::routes::{
    commands_page, create_command_endpoint, create_project_endpoint, delete_command_endpoint,
    delete_project_endpoint, home,
};

#[launch]
fn rocket() -> _ {
    // Before building the app, we make sure
    // the database exists.
    db::create_database().expect("Failed to init database, aborting ...");

    rocket::build()
        .mount(
            "/",
            routes![
                create_project_endpoint,
                commands_page,
                create_command_endpoint,
                delete_project_endpoint,
                delete_command_endpoint,
                home,
            ],
        )
        .attach(Template::fairing())
        .mount("/", FileServer::from("static"))
}
