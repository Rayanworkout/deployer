#[macro_use]
extern crate rocket;

mod db;
mod models;

// Template engine
// use rocket_dyn_templates::{Template, context};

// FileServer to serve static files
// use rocket::fs::FileServer;

use crate::db::crud::insert_project;
use crate::models::Project;

#[get("/")]
// fn index() -> Template {
//     Template::render("index", context! {})
// }

fn main() -> () {
    db::create_database().expect("Failed to init database, aborting ...");
    insert_project(&Project {
        name: String::from("dummy2q"),
        description: None,
        github_url: Some(String::from("https://github.com/rayanssse")),
    })
    .expect("");
}

// #[launch]
// fn rocket() -> _ {
//     // Before building the app, we make sure
//     // the database exists.
//     db::create_database().expect("Failed to init database, aborting ...");

//     rocket::build()
//         .mount("/", routes![index])
//         .attach(Template::fairing())
//         .mount("/", FileServer::from("static"))
// }
